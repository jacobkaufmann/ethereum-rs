use ethereum_types::U256;

/// Maximum number of items that can be held by a `Stack`.
pub const MAX_LEN: usize = 1024;

/// An error that may occur when performing operations on a `Stack`.
#[derive(Debug, PartialEq, Eq)]
pub enum StackError {
    Underflow,
    Overflow,
}

/// EVM stack.
pub struct Stack {
    items: [U256; MAX_LEN],
    len: usize,
}

impl Stack {
    /// Creates a `Stack`.
    pub fn new() -> Self {
        Self {
            items: [Default::default(); MAX_LEN],
            len: 0,
        }
    }

    /// Returns a reference to the item at position `pos` in the stack.
    pub fn get(&self, pos: usize) -> Option<&U256> {
        if pos >= self.len() {
            return None;
        }

        return Some(&self.items[self.len() - 1 - pos]);
    }

    /// Attempts to pop the top item from the stack.
    pub fn pop(&mut self) -> Result<U256, StackError> {
        if self.len() == 0 {
            return Err(StackError::Underflow);
        }

        let item = self.items[self.len() - 1];
        self.len -= 1;

        Ok(item)
    }

    /// Attempts to push an item onto the stack.
    pub fn push(&mut self, item: U256) -> Result<(), StackError> {
        if self.len() == MAX_LEN {
            return Err(StackError::Overflow);
        }

        self.items[self.len()] = item;
        self.len += 1;

        Ok(())
    }

    /// Attempts to swap the top (position zero) item in the stack with the position `pos` item.
    // TODO: Is there a better/clearer interface available here?
    pub fn swap_top(&mut self, pos: usize) -> Result<(), StackError> {
        if self.is_empty() || pos >= self.len() {
            return Err(StackError::Underflow);
        }

        let top: usize = self.len() - 1;
        let pos: usize = top - pos;
        self.items.swap(top, pos);

        Ok(())
    }

    /// Returns the number of items in the stack.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        assert_eq!(stack.get(0), None);

        let _ = stack.push(U256::MAX);
        assert_eq!(stack.get(0), Some(&U256::MAX));
    }

    #[test]
    fn test_push_overflow() {
        let mut stack = Stack::new();

        for _ in 0..MAX_LEN {
            assert_eq!(stack.push(U256::MAX), Ok(()));
        }

        assert_eq!(stack.push(U256::MAX), Err(StackError::Overflow));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();

        let _ = stack.push(U256::MAX);
        assert_eq!(stack.pop(), Ok(U256::MAX));
    }

    #[test]
    fn test_pop_underflow() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), Err(StackError::Underflow));
    }

    #[test]
    fn test_get() {
        let mut stack = Stack::new();

        assert_eq!(stack.get(0), None);

        let _ = stack.push(U256::MAX);
        assert_eq!(stack.get(0), Some(&U256::MAX));
    }

    #[test]
    fn test_swap_top() {
        let mut stack = Stack::new();

        // bottom [U256::MAX, U256::one(), U256::zero()] top
        let _ = stack.push(U256::MAX);
        let _ = stack.push(U256::one());
        let _ = stack.push(U256::zero());

        // bottom [U256::zero(), U256::one(), U256::MAX] top
        assert_eq!(stack.swap_top(2), Ok(()));
        assert_eq!(stack.get(0), Some(&U256::MAX));

        // bottom [U256::MAX, U256::one(), U256::zero()] top
        assert_eq!(stack.swap_top(2), Ok(()));
        assert_eq!(stack.get(0), Some(&U256::zero()));

        // bottom [U256::MAX, U256::zero(), U256::one()] top
        assert_eq!(stack.swap_top(1), Ok(()));
        assert_eq!(stack.get(0), Some(&U256::one()));
        assert_eq!(stack.get(1), Some(&U256::zero()));
        assert_eq!(stack.get(2), Some(&U256::MAX));
    }

    #[test]
    fn test_swap_top_underflow_empty() {
        let mut stack = Stack::new();

        assert_eq!(stack.swap_top(0), Err(StackError::Underflow));
    }

    #[test]
    fn test_swap_top_underflow_too_deep() {
        let mut stack = Stack::new();

        let _ = stack.push(U256::MAX);
        let _ = stack.push(U256::zero());

        assert_eq!(stack.swap_top(stack.len()), Err(StackError::Underflow));
    }

    #[test]
    fn test_len() {
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);

        for i in 0..MAX_LEN {
            let _ = stack.push(U256::MAX);
            assert_eq!(stack.len(), i + 1);
        }

        for i in 0..MAX_LEN {
            let _ = stack.pop();
            assert_eq!(stack.len(), MAX_LEN - 1 - i)
        }
    }

    #[test]
    fn test_is_empty() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        let _ = stack.push(U256::MAX);
        assert!(!stack.is_empty());

        let _ = stack.pop();
        assert!(stack.is_empty());
    }
}
