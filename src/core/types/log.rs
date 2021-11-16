use ethereum_types::{Address, H256};

/// An Ethereum transaction log entry.
pub struct Log {
    /// Address of the logger.
    address: Address,
    /// Log topics.
    topics: Vec<H256>,
    /// Data.
    data: Vec<u8>,
}
