use super::{
    blocks::{Block, BlockWithoutCanonicity},
    gen::BlockQueryInput,
    get_block_canonicity,
};
use crate::{
    block::{precomputed::PrecomputedBlock, store::BlockStore, BlockHash},
    canonicity::{store::CanonicityStore, Canonicity},
    command::{
        internal::{store::InternalCommandStore, InternalCommandWithData},
        store::UserCommandStore,
    },
    constants::*,
    snark_work::store::SnarkStore,
    store::IndexerStore,
    web::graphql::db,
};
use anyhow::Context as aContext;
use async_graphql::{Context, Enum, InputObject, Object, Result, SimpleObject};
use std::sync::Arc;

#[derive(SimpleObject, Debug)]
pub struct Feetransfer {
    pub state_hash: String,
    pub fee: u64,
    pub recipient: String,
    pub block_height: u32,
    pub date_time: String,

    #[graphql(name = "type")]
    pub feetransfer_kind: String,

    #[graphql(name = "epoch_num_internal_commands")]
    epoch_num_internal_commands: u32,

    #[graphql(name = "total_num_internal_commands")]
    total_num_internal_commands: u32,
}

#[derive(Debug)]
pub struct FeetransferWithMeta {
    /// Value canonicity
    pub canonical: bool,

    /// Value optional block
    pub block: Option<PrecomputedBlock>,

    /// Value feetranser
    pub feetransfer: Feetransfer,
}

#[Object]
impl FeetransferWithMeta {
    async fn canonical(&self) -> bool {
        self.canonical
    }

    #[graphql(flatten)]
    async fn feetransfer(&self) -> &Feetransfer {
        &self.feetransfer
    }

    async fn block_state_hash<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Block>> {
        let db = db(ctx);
        let epoch_num_blocks = db.get_block_production_epoch_count(None)?;
        let epoch_num_canonical_blocks = db.get_block_production_canonical_epoch_count(None)?;
        let epoch_num_supercharged_blocks =
            db.get_block_production_supercharged_epoch_count(None)?;
        let total_num_blocks = db.get_block_production_total_count()?;
        let total_num_canonical_blocks = db.get_block_production_canonical_total_count()?;
        let total_num_supercharged_blocks = db.get_block_production_supercharged_total_count()?;
        let epoch_num_user_commands = db.get_user_commands_epoch_count(None)?;
        let total_num_user_commands = db.get_user_commands_total_count()?;
        let epoch_num_slots_produced = db.get_epoch_slots_produced_count(None)?;

        if let Some(block) = self.block.clone() {
            let block_num_snarks = db
                .get_block_snarks_count(&block.state_hash())?
                .unwrap_or_default();
            let block_num_user_commands = db
                .get_block_user_commands_count(&block.state_hash())?
                .unwrap_or_default();
            let block_num_internal_commands = db
                .get_block_internal_commands_count(&block.state_hash())?
                .unwrap_or_default();
            Ok(Some(Block {
                block: BlockWithoutCanonicity::new(
                    &block,
                    self.canonical,
                    epoch_num_user_commands,
                    total_num_user_commands,
                ),
                canonical: self.canonical,
                epoch_num_blocks,
                epoch_num_canonical_blocks,
                epoch_num_supercharged_blocks,
                total_num_blocks,
                total_num_canonical_blocks,
                total_num_supercharged_blocks,
                block_num_snarks,
                block_num_user_commands,
                block_num_internal_commands,
                epoch_num_slots_produced,
                num_unique_block_producers_last_n_blocks: None,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(InputObject)]
pub struct FeetransferQueryInput {
    /// Value block height
    block_height: Option<u32>,

    /// Value block state hash
    block_state_hash: Option<BlockQueryInput>,

    /// Value canonical
    canonical: Option<bool>,

    /// Value recipient
    recipient: Option<String>,

    /// Value block height greater than
    #[graphql(name = "blockHeight_gt")]
    pub block_height_gt: Option<u32>,

    /// Value block height greater than or equal to
    #[graphql(name = "blockHeight_gte")]
    pub block_height_gte: Option<u32>,

    /// Value block height less than
    #[graphql(name = "blockHeight_lt")]
    pub block_height_lt: Option<u32>,

    /// Value block height less than or equal to
    #[graphql(name = "blockHeight_lte")]
    pub block_height_lte: Option<u32>,

    /// Value and
    and: Option<Vec<FeetransferQueryInput>>,

    /// Value or
    or: Option<Vec<FeetransferQueryInput>>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FeetransferSortByInput {
    #[graphql(name = "BLOCKHEIGHT_ASC")]
    BlockHeightAsc,

    #[graphql(name = "BLOCKHEIGHT_DESC")]
    BlockHeightDesc,
}

#[derive(Default)]
pub struct FeetransferQueryRoot;

#[Object]
impl FeetransferQueryRoot {
    async fn feetransfers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query: Option<FeetransferQueryInput>,
        sort_by: Option<FeetransferSortByInput>,
        #[graphql(default = 100)] limit: usize,
    ) -> Result<Vec<FeetransferWithMeta>> {
        let db = db(ctx);
        let epoch_num_internal_commands = db.get_internal_commands_epoch_count(None)?;
        let total_num_internal_commands = db.get_internal_commands_total_count()?;

        // state_hash query
        if let Some(state_hash) = query
            .as_ref()
            .and_then(|f| f.block_state_hash.as_ref())
            .and_then(|f| f.state_hash.clone())
        {
            return Ok(get_fee_transfers_for_state_hash(
                db,
                &query,
                &state_hash.into(),
                sort_by,
                limit,
                epoch_num_internal_commands,
                total_num_internal_commands,
            ));
        }

        // TODO
        // block height bounded query
        if query.as_ref().map_or(false, |q| {
            q.block_height_gt.is_some()
                || q.block_height_gte.is_some()
                || q.block_height_lt.is_some()
                || q.block_height_lte.is_some()
        }) {
            let mut feetransfers = Vec::new();
            let (min, max) = {
                let FeetransferQueryInput {
                    block_height_gt,
                    block_height_gte,
                    block_height_lt,
                    block_height_lte,
                    ..
                } = query.as_ref().expect("query will contain a value");
                let min_bound = match (*block_height_gte, *block_height_gt) {
                    (Some(gte), Some(gt)) => std::cmp::max(gte, gt + 1),
                    (Some(gte), None) => gte,
                    (None, Some(gt)) => gt + 1,
                    (None, None) => 1,
                };

                let max_bound = match (*block_height_lte, *block_height_lt) {
                    (Some(lte), Some(lt)) => std::cmp::min(lte, lt - 1),
                    (Some(lte), None) => lte,
                    (None, Some(lt)) => lt - 1,
                    (None, None) => db.get_best_block_height()?.unwrap(),
                };
                (min_bound, max_bound)
            };

            let mut block_heights: Vec<u32> = (min..=max).collect();
            let sort_by = sort_by.unwrap_or(FeetransferSortByInput::BlockHeightDesc);
            if sort_by == FeetransferSortByInput::BlockHeightDesc {
                block_heights.reverse()
            }

            'outer: for height in block_heights {
                for state_hash in db.get_blocks_at_height(height)?.iter() {
                    let canonical = get_block_canonicity(db, state_hash);
                    let block = db
                        .get_block(state_hash)?
                        .with_context(|| format!("block missing from store {state_hash}"))
                        .unwrap()
                        .0;
                    for internal_cmd in InternalCommandWithData::from_precomputed(&block) {
                        let ft = Feetransfer::from((
                            internal_cmd,
                            epoch_num_internal_commands,
                            total_num_internal_commands,
                        ));
                        let feetransfer_with_meta = FeetransferWithMeta {
                            canonical,
                            feetransfer: ft,
                            block: Some(block.clone()),
                        };
                        if query
                            .as_ref()
                            .map_or(true, |q| q.matches(&feetransfer_with_meta))
                        {
                            feetransfers.push(feetransfer_with_meta);
                            if feetransfers.len() == limit {
                                break 'outer;
                            }
                        }
                    }
                }
            }
            return Ok(feetransfers);
        }

        // recipient query
        if let Some(recipient) = query.as_ref().and_then(|q| q.recipient.clone()) {
            let mut fee_transfers: Vec<FeetransferWithMeta> = vec![];
            let mut offset = 0;
            let mut last_fee_transfer_len = fee_transfers.len();
            if let Some(max_internal_commands) =
                db.get_pk_num_internal_commands(&recipient.clone().into())?
            {
                loop {
                    let end_index =
                        std::cmp::min(offset + limit - 1, max_internal_commands as usize);
                    if end_index <= offset || last_fee_transfer_len >= limit {
                        // there is no more data to be gathered because either:
                        // a. no more internal commands exist in this range for this PK
                        // b. the gathered data is at or exceeds the limit
                        break;
                    }
                    fee_transfers.append(
                        &mut db
                            .get_internal_commands_public_key(
                                &recipient.clone().into(),
                                offset,
                                end_index,
                            )?
                            .into_iter()
                            .map(|internal_command| {
                                let ft = Feetransfer::from((
                                    internal_command,
                                    epoch_num_internal_commands,
                                    total_num_internal_commands,
                                ));
                                let state_hash = BlockHash::from(ft.state_hash.clone());
                                let pcb = db.get_block(&state_hash).unwrap().unwrap().0;
                                let canonical = get_block_canonicity(db, &state_hash);
                                FeetransferWithMeta {
                                    canonical,
                                    feetransfer: ft,
                                    block: Some(pcb),
                                }
                            })
                            .filter(|ft| query.as_ref().map_or(true, |q| q.matches(ft)))
                            .collect(),
                    );

                    offset += limit;
                    last_fee_transfer_len = fee_transfers.len();
                }
            }
            fee_transfers.truncate(limit);
            return Ok(fee_transfers);
        }
        get_fee_transfers(
            db,
            query,
            sort_by,
            limit,
            epoch_num_internal_commands,
            total_num_internal_commands,
        )
    }
}

fn get_fee_transfers(
    db: &Arc<IndexerStore>,
    query: Option<FeetransferQueryInput>,
    sort_by: Option<FeetransferSortByInput>,
    limit: usize,
    epoch_num_internal_commands: u32,
    total_num_internal_commands: u32,
) -> Result<Vec<FeetransferWithMeta>> {
    let mut fee_transfers = Vec::new();
    let mode = if let Some(FeetransferSortByInput::BlockHeightAsc) = sort_by {
        speedb::IteratorMode::Start
    } else {
        speedb::IteratorMode::End
    };

    for (_, value) in db.internal_commands_block_height_iterator(mode).flatten() {
        let internal_command = serde_json::from_slice::<InternalCommandWithData>(&value)?;
        let ft = Feetransfer::from((
            internal_command,
            epoch_num_internal_commands,
            total_num_internal_commands,
        ));
        let state_hash = BlockHash::from(ft.state_hash.clone());
        let canonical = get_block_canonicity(db, &state_hash);
        let pcb = db.get_block(&state_hash).unwrap().unwrap().0;
        let feetransfer_with_meta = FeetransferWithMeta {
            canonical,
            feetransfer: ft,
            block: Some(pcb),
        };

        if query
            .as_ref()
            .map_or(true, |q| q.matches(&feetransfer_with_meta))
        {
            fee_transfers.push(feetransfer_with_meta);
        }

        if fee_transfers.len() >= limit {
            break;
        }
    }
    Ok(fee_transfers)
}

fn get_fee_transfers_for_state_hash(
    db: &Arc<IndexerStore>,
    query: &Option<FeetransferQueryInput>,
    state_hash: &BlockHash,
    sort_by: Option<FeetransferSortByInput>,
    limit: usize,
    epoch_num_internal_commands: u32,
    total_num_internal_commands: u32,
) -> Vec<FeetransferWithMeta> {
    let pcb = match db.get_block(state_hash) {
        Ok(Some(pcb)) => pcb.0,
        _ => return vec![],
    };
    let canonical = match db.get_block_canonicity(state_hash) {
        Ok(Some(canonicity)) => matches!(canonicity, Canonicity::Canonical),
        _ => false,
    };
    match db.get_internal_commands(state_hash) {
        Ok(internal_commands) => {
            let mut internal_commands: Vec<FeetransferWithMeta> = internal_commands
                .into_iter()
                .map(|ft| FeetransferWithMeta {
                    canonical,
                    feetransfer: Feetransfer::from((
                        ft,
                        epoch_num_internal_commands,
                        total_num_internal_commands,
                    )),
                    block: Some(pcb.clone()),
                })
                .filter(|ft| query.as_ref().map_or(true, |q| q.matches(ft)))
                .collect();

            if let Some(sort_by) = sort_by {
                match sort_by {
                    FeetransferSortByInput::BlockHeightAsc => {
                        internal_commands.sort_by(|a, b| {
                            a.feetransfer.block_height.cmp(&b.feetransfer.block_height)
                        });
                    }
                    FeetransferSortByInput::BlockHeightDesc => {
                        internal_commands.sort_by(|a, b| {
                            b.feetransfer.block_height.cmp(&a.feetransfer.block_height)
                        });
                    }
                }
            }

            internal_commands.truncate(limit);
            internal_commands
        }
        Err(_) => vec![],
    }
}

impl From<(InternalCommandWithData, u32, u32)> for Feetransfer {
    fn from(int_cmd: (InternalCommandWithData, u32, u32)) -> Self {
        match int_cmd.0 {
            InternalCommandWithData::FeeTransfer {
                receiver,
                amount,
                state_hash,
                kind,
                date_time,
                block_height,
                ..
            } => Self {
                state_hash: state_hash.0,
                fee: amount,
                recipient: receiver.0,
                feetransfer_kind: kind.to_string(),
                block_height,
                date_time: millis_to_iso_date_string(date_time),
                epoch_num_internal_commands: int_cmd.1,
                total_num_internal_commands: int_cmd.2,
            },
            InternalCommandWithData::Coinbase {
                receiver,
                amount,
                state_hash,
                kind,
                date_time,
                block_height,
            } => Self {
                state_hash: state_hash.0,
                fee: amount,
                recipient: receiver.0,
                feetransfer_kind: kind.to_string(),
                block_height,
                date_time: millis_to_iso_date_string(date_time),
                epoch_num_internal_commands: int_cmd.1,
                total_num_internal_commands: int_cmd.2,
            },
        }
    }
}

impl FeetransferQueryInput {
    pub fn matches(&self, ft: &FeetransferWithMeta) -> bool {
        let Self {
            block_height_gt,
            block_height_gte,
            block_height_lt,
            block_height_lte,
            block_height,
            block_state_hash,
            canonical,
            recipient,
            and,
            or,
        } = self;

        if let Some(canonical) = canonical.as_ref() {
            if ft.canonical != *canonical {
                return false;
            }
        }
        if let Some(recipient) = recipient.as_ref() {
            if ft.feetransfer.recipient != *recipient {
                return false;
            }
        }

        let pcb = ft.block.as_ref().expect("block will exist");
        let blockchain_length = pcb.blockchain_length();
        if let Some(height) = block_height.as_ref() {
            if blockchain_length != *height {
                return false;
            }
        }

        // block_height_gt(e) & block_height_lt(e)
        if let Some(height) = block_height_gt {
            if blockchain_length <= *height {
                return false;
            }
        }
        if let Some(height) = block_height_gte {
            if blockchain_length < *height {
                return false;
            }
        }
        if let Some(height) = block_height_lt {
            if blockchain_length >= *height {
                return false;
            }
        }
        if let Some(height) = block_height_lte {
            if blockchain_length > *height {
                return false;
            }
        }
        if let Some(block_query_input) = block_state_hash.as_ref() {
            if let Some(state_hash) = block_query_input.state_hash.as_ref() {
                if ft.feetransfer.state_hash != *state_hash {
                    return false;
                }
            }
        }
        if let Some(query) = and.as_ref() {
            if !(query.iter().all(|and| and.matches(ft))) {
                return false;
            }
        }
        if let Some(query) = or.as_ref() {
            if !query.is_empty() && !(query.iter().any(|or| or.matches(ft))) {
                return false;
            }
        }
        true
    }
}
