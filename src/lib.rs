
pub mod stack {
    use std::fmt;
    use std::vec::Vec;

    /// Error type indicating the stack is full. 
    #[derive(Debug, PartialEq)]
    pub struct StackFullError;

    impl fmt::Display for StackFullError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "stack is full")
        }
    }

    /// A heap allocated stack initialized with a specific size.
    pub struct Stack<T> {
        pub(crate) stack: Vec<T> // visible to crate for unit testing 
    } 

    impl<T> Stack<T> {
        /// Create a new stack with a capacity of `size`. 
        pub fn new(size: usize) -> Self {
            Stack {
                stack: Vec::with_capacity(size),
            }
        }
        
        /// Push a T onto the stack if the stack is not full. If the 
        /// stack is full, a `StackFullError` is returned.
        pub fn push(&mut self, val: T) -> Result<(), StackFullError> {
            if self.stack.len() < self.stack.capacity() {
                self.stack.push(val);
                Ok(())
            } else {
                Err(StackFullError)
            }
        }

        /// Removes an element from the stack if one exists. 
        /// Returns `Some(T)` or `None` if the stack is empty.
        pub fn pop(&mut self) -> Option<T> {
            self.stack.pop()
        }

        /// Returns the current size of the stack as a `usize`.
        pub fn size(&self) -> usize {
            self.stack.len()
        }
    }
}

#[cfg(test)]
mod test_stack {
    use crate::stack;

    #[test]
    fn stack_push() {
        let mut stack = stack::Stack::new(5);
        
        let ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        let last_elem = stack.stack.last().clone();
        assert_eq!(last_elem, Some(&542u32));
    }

    #[test]
    fn stack_push_when_full_returns_stackfullerror() {
        let mut stack = stack::Stack::new(5);

        let mut ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(542u32);
        assert_eq!(ret, Err(stack::StackFullError));
    }

    #[test]
    fn stack_pop() {
        let mut stack = stack::Stack::new(5);

        let ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        let popped_val = stack.pop();
        assert_eq!(popped_val, Some(542u32));
    }

    #[test]
    fn stack_pop_when_empty_retuns_none() {
        let mut stack = stack::Stack::<u32>::new(5);

        let ret = stack.pop();
        assert_eq!(ret, None);
    }

    #[test]
    fn stack_new_returns_empty_stack() {
        let stack = stack::Stack::<u32>::new(5);

        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn stack_new_returns_stack_with_correct_capacity() {
        let stack = stack::Stack::<u32>::new(5);

        assert_eq!(stack.stack.capacity(), 5);
    }
}
