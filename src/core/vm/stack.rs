use ethereum_types::U256;

/// Maximum number of items that can be held by a `Stack`.
pub const MAX_LEN: usize = 1024;

/// An error that may occur when performing operations on a `Stack`.
pub enum StackError {
    Underflow,
    Overflow,
}

/// EVM stack.
pub struct Stack(Vec<U256>);

impl Stack {
    /// Creates a `Stack` with a capacity equal to `MAX_LEN`.
    pub fn new() -> Self {
        Self(Vec::with_capacity(MAX_LEN))
    }

    /// Returns a reference to the item at position `pos` in the stack.
    pub fn get(&self, pos: usize) -> Option<&U256> {
        self.0.get(self.len() - 1 - pos)
    }

    /// Attempts to pop the top item from the stack.
    pub fn pop(&mut self) -> Result<U256, StackError> {
        match self.0.pop() {
            Some(item) => Ok(item),
            None => Err(StackError::Underflow),
        }
    }

    /// Attempts to push an item onto the stack.
    pub fn push(&mut self, item: U256) -> Result<(), StackError> {
        if self.0.len() == MAX_LEN {
            return Err(StackError::Overflow);
        }

        self.0.push(item);

        Ok(())
    }

    /// Attempts to swap the top (position zero) item in the stack with the position `pos` item.
    pub fn swap_top(&mut self, pos: usize) -> Result<(), StackError> {
        if self.is_empty() || pos >= self.len() {
            return Err(StackError::Underflow);
        }

        let top: usize = self.len() - 1;
        let pos: usize = top - pos;
        self.0.swap(top, pos);

        Ok(())
    }

    /// Returns the number of items in the stack.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
