use crate::helpers::setup_new_db_dir;
use mina_indexer::{
    block::{genesis::GenesisBlock, store::BlockStore},
    constants::*,
    ledger::genesis::parse_file,
    server::IndexerVersion,
    state::IndexerState,
    store::IndexerStore,
};
use std::{path::PathBuf, sync::Arc};

#[test]
fn block_added() -> anyhow::Result<()> {
    let store_dir = setup_new_db_dir("block-store-genesis")?;
    let indexer_store = Arc::new(IndexerStore::new(store_dir.path())?);
    let genesis_ledger_path = &PathBuf::from("./data/genesis_ledgers/mainnet.json");
    let genesis_root = parse_file(genesis_ledger_path)?;

    let indexer = IndexerState::new(
        genesis_root.into(),
        IndexerVersion::default(),
        indexer_store,
        MAINNET_CANONICAL_THRESHOLD,
        MAINNET_TRANSITION_FRONTIER_K,
        false,
    )?;

    assert_eq!(
        indexer
            .indexer_store
            .unwrap()
            .get_block(&MAINNET_GENESIS_HASH.into())
            .unwrap()
            .map(|b| b.0),
        Some(GenesisBlock::new_v1().unwrap().to_precomputed())
    );
    Ok(())
}
