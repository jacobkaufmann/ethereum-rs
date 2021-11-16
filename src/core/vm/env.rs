use crate::core::types::block::BlockHeader;
use ethereum_types::{Address, U256};

/// EVM execution environment.
pub struct ExecutionEnv {
    /// Address of the account that owns the code that is executing.
    account: Address,
    /// Sender address of the transaction that originated this execution.
    origin: Address,
    /// Gas price in the transaction that originated this execution.
    gas_price: U256,
    /// Input data to this execution.
    data: Vec<u8>,
    /// Address of the account that caused the code to execute.
    sender: Address,
    /// Value (in Wei) passed to `account` as part of the same procedure as execution.
    value: U256,
    /// Machine code to be executed.
    machine_code: Vec<u8>,
    /// Block header of the present block.
    header: BlockHeader,
    /// Depth of the present message call or contract creation. The number of CALLs
    /// or CREATE(2)s being executed at present.
    depth: U256,
    /// Permission to make modifications to the state.
    write_access: bool,
}
