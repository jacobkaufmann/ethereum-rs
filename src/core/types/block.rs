use super::transaction::Transaction;
use ethereum_types::{Address, Bloom, H256, U256, U64};

/// An Ethereum block header.
pub struct BlockHeader {
    parent_hash: H256,
    omners_hash: H256,
    beneficiary: Address,
    state_root: H256,
    transactions_root: H256,
    receipts_root: H256,
    logs_bloom: Bloom,
    difficulty: U256,
    number: U256,
    gas_limit: U256,
    gas_used: U256,
    timestamp: U256,
    extra_data: Option<[u8; 32]>,
    mix_hash: H256,
    nonce: U64,
}

/// An Ethereum block.
pub struct Block {
    header: BlockHeader,
    omners: Vec<BlockHeader>,
    transactions: Vec<Transaction>,
}
