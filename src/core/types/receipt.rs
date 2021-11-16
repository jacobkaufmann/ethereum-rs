use super::log::Log;
use ethereum_types::{Bloom, U256};

/// An Ethereum transaction receipt.
struct Receipt {
    /// Status code of the transaction.
    status_code: U256,
    /// Cumulative gas used in the block containing the transaction receipt as of
    /// the completion of the execution of the transaction.
    cumulative_gas_used: U256,
    /// Set of logs created through the execution of the transaction.
    logs: Vec<Log>,
    /// Bloom filter composed from information in `logs`.
    logs_bloom: Bloom,
}
