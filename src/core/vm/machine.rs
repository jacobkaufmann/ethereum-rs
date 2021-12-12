use ethereum_types::U256;

use super::memory::Memory;
use super::stack::Stack;

/// EVM machine state.
struct MachineState {
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
    pub fn new() -> Self {
        Self {
            gas_available: U256::from(0),
            program_counter: U256::from(0),
            memory: Memory::new(),
            words_in_memory: U256::from(0),
            stack: Stack::new(),
        }
    }
}
