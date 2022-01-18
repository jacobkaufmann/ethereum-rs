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
    /// Value (in Wei) passed to account as part of the same procedure as execution.
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

impl ExecutionEnv {
    /// Returns the account that owns the code that is executing.
    pub fn account(&self) -> &Address {
        &self.account
    }

    /// Returns the sender address of the transaction that originated this execution.
    pub fn origin(&self) -> &Address {
        &self.origin
    }

    /// Returns the gas price in the transaction that originated this execution.
    pub fn gas_price(&self) -> &U256 {
        &self.gas_price
    }

    /// Returns the input data to this execution.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Returns the address of the account that caused the code to execute.
    pub fn sender(&self) -> &Address {
        &self.sender
    }

    /// Returns the value (in Wei) passed to account as part of the same procedure as execution.
    pub fn value(&self) -> &U256 {
        &self.value
    }

    /// Returns the machine code to be executed.
    pub fn machine_code(&self) -> &Vec<u8> {
        &self.machine_code
    }

    /// Returns the block header of the current block.
    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    /// Returns the depth of the present message call or contract creation. The number of CALLs or
    /// CREATE(2)s being executed at present.
    pub fn depth(&self) -> &U256 {
        &self.depth
    }

    /// Returns the permission to make modifications to the state.
    pub fn write_access(&self) -> bool {
        self.write_access
    }
}
