use ethereum_types::U256;

use super::memory::Memory;
use super::stack::Stack;

/// EVM machine state.
pub struct MachineState {
    /// Gas available.
    gas_available: U256,
    /// Program counter.
    program_counter: U256,
    /// Memory contents.
    memory: Memory,
    /// Active number of words in memory.
    words_in_memory: U256,
    /// Stack contents.
    stack: Stack,
}

impl MachineState {
    /// Returns a new machine state.
    pub fn new() -> Self {
        Self {
            gas_available: U256::from(0),
            program_counter: U256::from(0),
            memory: Memory::new(),
            words_in_memory: U256::from(0),
            stack: Stack::new(),
        }
    }

    /// Returns available gas.
    pub fn gas_available(&self) -> &U256 {
        &self.gas_available
    }

    /// Returns program counter.
    pub fn program_counter(&self) -> &U256 {
        &self.program_counter
    }

    /// Returns memory.
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Returns words in memory.
    pub fn words_in_memory(&self) -> &U256 {
        &self.words_in_memory
    }

    /// Returns stack.
    pub fn stack(&self) -> &Stack {
        &self.stack
    }
}
