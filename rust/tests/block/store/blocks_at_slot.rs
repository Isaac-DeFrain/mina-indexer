use crate::helpers::setup_new_db_dir;
use mina_indexer::{
    block::{
        parser::BlockParser,
        precomputed::{PcbVersion, PrecomputedBlock},
        store::BlockStore,
    },
    constants::*,
    store::IndexerStore,
};
use std::path::PathBuf;

#[test]
fn add_and_get() -> anyhow::Result<()> {
    let store_dir = setup_new_db_dir("blocks-at-slot")?;
    let block_dir = &PathBuf::from("./tests/data/sequential_blocks");

    let db = IndexerStore::new(store_dir.path())?;
    let mut bp = BlockParser::new_with_canonical_chain_discovery(
        block_dir,
        PcbVersion::V1,
        MAINNET_CANONICAL_THRESHOLD,
        BLOCK_REPORTING_FREQ_NUM,
    )?;

    while let Some((block, _)) = bp.next_block()? {
        let block: PrecomputedBlock = block.into();
        db.add_block(&block)?;
    }

    assert_eq!(db.get_blocks_at_slot(155140)?.len(), 3);
    Ok(())
}
