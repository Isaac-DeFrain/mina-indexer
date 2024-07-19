use super::{account::AccountBalanceUpdate, column_families::ColumnFamilyHelpers, from_be_bytes};
use crate::{
    block::{store::BlockStore, BlockHash},
    constants::MAINNET_GENESIS_HASH,
    ledger::{
        account::{Account, Amount},
        public_key::PublicKey,
    },
    store::{
        account::{AccountStore, DBAccountBalanceUpdate},
        fixed_keys::FixedKeys,
        to_be_bytes, u64_prefix_key, IndexerStore,
    },
};
use log::trace;
use speedb::{DBIterator, IteratorMode};

impl AccountStore for IndexerStore {
    fn reorg_account_updates(
        &self,
        old_best_tip: &BlockHash,
        new_best_tip: &BlockHash,
    ) -> anyhow::Result<DBAccountBalanceUpdate> {
        trace!(
            "Getting common ancestor account balance updates:\n  old: {}\n  new: {}",
            old_best_tip,
            new_best_tip
        );

        // follows the old best tip back to the common ancestor
        let mut a = old_best_tip.clone();
        let mut unapply = vec![];

        // follows the new best tip back to the common ancestor
        let mut b = new_best_tip.clone();
        let mut apply = vec![];

        let a_length = self.get_block_height(&a)?.expect("a has a length");
        let b_length = self.get_block_height(&b)?.expect("b has a length");

        // bring b back to the same height as a
        let genesis_state_hashes: Vec<BlockHash> = vec![MAINNET_GENESIS_HASH.into()];
        for _ in 0..b_length.saturating_sub(a_length) {
            // check if there's a previous block
            if genesis_state_hashes.contains(&b) {
                break;
            }

            apply.append(&mut self.get_block_balance_updates(&b)?.unwrap());
            b = self.get_block_parent_hash(&b)?.expect("b has a parent");
        }

        // find the common ancestor
        let mut a_prev = self.get_block_parent_hash(&a)?.expect("a has a parent");
        let mut b_prev = self.get_block_parent_hash(&b)?.expect("b has a parent");

        while a != b && !genesis_state_hashes.contains(&a) {
            // add blocks to appropriate collection
            unapply.append(&mut self.get_block_balance_updates(&a)?.unwrap());
            apply.append(&mut self.get_block_balance_updates(&b)?.unwrap());

            // descend
            a = a_prev;
            b = b_prev;

            a_prev = self.get_block_parent_hash(&a)?.expect("a has a parent");
            b_prev = self.get_block_parent_hash(&b)?.expect("b has a parent");
        }

        apply.reverse();
        Ok(<DBAccountBalanceUpdate>::new(apply, unapply))
    }

    fn get_block_balance_updates(
        &self,
        state_hash: &BlockHash,
    ) -> anyhow::Result<Option<Vec<AccountBalanceUpdate>>> {
        trace!("Getting block balance updates for {state_hash}");
        Ok(self
            .database
            .get_pinned_cf(self.blocks_account_updates_cf(), state_hash.0.as_bytes())?
            .and_then(|bytes| serde_json::from_slice(&bytes).ok()))
    }

    fn update_account_balances(
        &self,
        state_hash: &BlockHash,
        updates: &DBAccountBalanceUpdate,
    ) -> anyhow::Result<()> {
        use AccountBalanceUpdate::*;
        trace!("Updating account balances {state_hash}");
        fn count(updates: &[AccountBalanceUpdate]) -> i32 {
            updates.iter().fold(0, |acc, update| match update {
                CreateAccount(_) => acc + 1,
                RemoveAccount(_) => acc - 1,
                Payment(_) => acc,
            })
        }
        self.update_num_accounts(count(&updates.apply) - count(&updates.unapply))?;

        // update balances
        for (pk, amount) in <DBAccountBalanceUpdate>::balance_updates(updates) {
            if let Some(amount) = amount {
                let account = self.get_account(&pk)?.unwrap_or(Account::empty(pk.clone()));
                let amt: Amount = amount.unsigned_abs().into();
                let account = if amount > 0 {
                    Account {
                        balance: account.balance.add(&amt),
                        ..account
                    }
                } else {
                    Account {
                        balance: account.balance.sub(&amt),
                        ..account
                    }
                };
                self.update_account(&pk, Some(account))?;
            } else {
                // remove account
                self.update_account(&pk, None)?;
            }
        }
        Ok(())
    }

    fn add_pk_delegate(&self, pk: &PublicKey, delegate: &PublicKey) -> anyhow::Result<()> {
        trace!("Adding pk {pk} delegate {delegate}");
        let num = self.get_num_pk_delegations(pk)?;
        self.database.put_cf(
            self.account_num_delegations(),
            pk.0.as_bytes(),
            to_be_bytes(num + 1),
        )?;

        let mut key = pk.clone().to_bytes();
        key.append(&mut to_be_bytes(num));
        self.database
            .put_cf(self.account_delegations(), key, delegate.0.as_bytes())?;
        Ok(())
    }

    fn get_num_pk_delegations(&self, pk: &PublicKey) -> anyhow::Result<u32> {
        Ok(self
            .database
            .get_cf(self.account_num_delegations(), pk.0.as_bytes())?
            .map_or(0, from_be_bytes))
    }

    fn get_pk_delegation(&self, pk: &PublicKey, idx: u32) -> anyhow::Result<Option<PublicKey>> {
        let mut key = pk.clone().to_bytes();
        key.append(&mut to_be_bytes(idx));
        Ok(self
            .database
            .get_cf(self.account_delegations(), key)?
            .and_then(|bytes| PublicKey::from_bytes(&bytes).ok()))
    }

    fn remove_pk_delegate(&self, pk: PublicKey) -> anyhow::Result<()> {
        trace!("Removing pk {pk} delegate");
        let idx = self.get_num_pk_delegations(&pk)?;
        if idx > 0 {
            self.database.put_cf(
                self.account_num_delegations(),
                pk.0.as_bytes(),
                to_be_bytes(idx - 1),
            )?;

            let mut key = pk.to_bytes();
            key.append(&mut to_be_bytes(idx - 1));
            self.database.delete_cf(self.account_delegations(), key)?;
        }
        Ok(())
    }

    fn update_num_accounts(&self, adjust: i32) -> anyhow::Result<()> {
        use std::cmp::Ordering::*;
        match adjust.cmp(&0) {
            Equal => (),
            Greater => {
                let old = self
                    .database
                    .get(Self::TOTAL_NUM_ACCOUNTS_KEY)?
                    .map_or(0, from_be_bytes);
                self.database.put(
                    Self::TOTAL_NUM_ACCOUNTS_KEY,
                    old.saturating_add(adjust.unsigned_abs()).to_be_bytes(),
                )?;
            }
            Less => {
                let old = self
                    .database
                    .get(Self::TOTAL_NUM_ACCOUNTS_KEY)?
                    .map_or(0, from_be_bytes);
                self.database.put(
                    Self::TOTAL_NUM_ACCOUNTS_KEY,
                    old.saturating_sub(adjust.unsigned_abs()).to_be_bytes(),
                )?;
            }
        }
        Ok(())
    }

    fn get_num_accounts(&self) -> anyhow::Result<Option<u32>> {
        Ok(self
            .database
            .get(Self::TOTAL_NUM_ACCOUNTS_KEY)?
            .map(from_be_bytes))
    }

    fn update_account(&self, pk: &PublicKey, account: Option<Account>) -> anyhow::Result<()> {
        if account.is_none() {
            // delete stale data
            if let Some(account) = self.get_account(pk)? {
                self.database
                    .delete_cf(self.accounts_cf(), pk.0.as_bytes())?;
                self.database.delete_cf(
                    self.accounts_balance_sort_cf(),
                    u64_prefix_key(account.balance.0, &pk.0),
                )?;
                return Ok(());
            }
        }

        // update account
        let account = account.unwrap();
        let balance = account.balance.0;
        if let Some(Account { balance, .. }) = self.get_account(pk)? {
            // delete stale balance sorting data
            self.database.delete_cf(
                self.accounts_balance_sort_cf(),
                u64_prefix_key(balance.0, &pk.0),
            )?;
        }
        self.database.put_cf(
            self.accounts_cf(),
            pk.0.as_bytes(),
            serde_json::to_vec(&account)?,
        )?;

        // add: {balance}{pk} -> _
        self.database.put_cf(
            self.accounts_balance_sort_cf(),
            u64_prefix_key(balance, &pk.0),
            b"",
        )?;
        Ok(())
    }

    fn set_block_balance_updates(
        &self,
        state_hash: &BlockHash,
        balance_updates: &[AccountBalanceUpdate],
    ) -> anyhow::Result<()> {
        trace!("Setting block balance updates for {state_hash}");
        self.database.put_cf(
            self.blocks_account_updates_cf(),
            state_hash.0.as_bytes(),
            serde_json::to_vec(balance_updates)?,
        )?;
        Ok(())
    }

    fn get_account(&self, pk: &PublicKey) -> anyhow::Result<Option<Account>> {
        trace!("Getting account balance {pk}");
        Ok(self
            .database
            .get_cf(self.accounts_cf(), pk.0.as_bytes())?
            .and_then(|bytes| serde_json::from_slice(&bytes).ok()))
    }

    ///////////////
    // Iterators //
    ///////////////

    fn account_balance_iterator<'a>(&'a self, mode: IteratorMode) -> DBIterator<'a> {
        self.database
            .iterator_cf(self.accounts_balance_sort_cf(), mode)
    }
}
