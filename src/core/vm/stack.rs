use ethereum_types::H256;

/// Maximum number of items that can be held by a `Stack`.
pub const MAX_SIZE: usize = 1024;

/// An error that may occur when performing operations on a `Stack`.
pub enum StackError {
    Underflow,
    Overflow,
}

/// EVM stack.
pub struct Stack {
    items: Vec<H256>,
}

impl Stack {
    /// Creates a `Stack` with a capacity equal to `MAX_SIZE`.
    pub fn new() -> Self {
        Stack {
            items: Vec::with_capacity(MAX_SIZE),
        }
    }

    /// Attempts to pop the top item from the stack.
    pub fn pop(&mut self, items: usize) -> Result<Vec<H256>, StackError> {
        if items > self.items.len() {
            return Err(StackError::Underflow);
        }

        let mut popped = vec![];
        while popped.len() < items {
            popped.push(self.items.pop().unwrap())
        }

        Ok(popped)
    }

    /// Attempts to push an item onto the stack.
    pub fn push(&mut self, items: Vec<H256>) -> Result<(), StackError> {
        if self.items.len() + items.len() > MAX_SIZE {
            return Err(StackError::Overflow);
        }

        for item in items {
            self.items.push(item);
        }

        Ok(())
    }
}
