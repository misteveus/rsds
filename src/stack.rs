
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

/// A heap allocated stack that holds elements of type `T`.
pub struct Stack<T> {
    stack: Vec<T>
} 

impl<T> Stack<T> {
    /// Create a new stack with a max capacity of `size`. 
    /// ```
    /// use rsds::stack::Stack;
    /// 
    /// // Initialize an emply stack with a max capacity of 5 `u32`'s.
    /// let mut s = Stack::<u32>::new(5);
    /// ```
    pub fn new(size: usize) -> Self {
        Stack {
            stack: Vec::with_capacity(size),
        }
    }
    
    /// Push a value onto the stack if the stack is not full. If the 
    /// stack is full, a `StackFullError` is returned.
    /// ```
    /// use rsds::stack::{Stack, StackFullError};
    /// 
    /// let mut s = Stack::<u32>::new(5);
    /// 
    /// // fill the stack
    /// s.push(1u32);
    /// s.push(2u32);
    /// s.push(3u32);
    /// s.push(4u32);
    /// s.push(5u32);
    /// 
    /// // stack is full and should return a StackFullError
    /// let ret = s.push(6u32);
    /// assert_eq!(ret, Err(StackFullError));
    /// ```
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
    /// ```
    /// use rsds::stack::Stack;
    /// 
    /// let mut s = Stack::<u32>::new(5);
    /// 
    ///
    /// s.push(1u32);
    /// s.push(2u32);
    /// 
    /// // remove values from stack
    /// let mut ret = s.pop();
    /// assert_eq!(ret, Some(2u32));
    /// 
    /// ret = s.pop();
    /// assert_eq!(ret, Some(1u32));
    /// 
    /// // stack empty and should return None
    /// ret = s.pop();
    /// assert_eq!(ret, None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Returns the current size of the stack as a `usize`.
    ///     /// ```
    /// use rsds::stack::Stack;
    /// 
    /// let mut s = Stack::<u32>::new(5);
    /// 
    ///
    /// s.push(1u32);
    /// s.push(2u32);
    /// 
    /// // remove values from stack
    /// let mut ret = s.pop();
    /// assert_eq!(ret, Some(2u32));
    /// 
    /// ret = s.pop();
    /// assert_eq!(ret, Some(1u32));
    /// 
    /// // stack is empty and should return None
    /// ret = s.pop();
    /// assert_eq!(ret, None);
    /// ```
    pub fn size(&self) -> usize {
        self.stack.len()
    }
}


#[cfg(test)]
mod test_stack {
    use crate::stack::*;

    #[test]
    fn stack_push() {
        let mut stack = Stack::new(5);
        
        let ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        let last_elem = stack.stack.last().clone();
        assert_eq!(last_elem, Some(&542u32));
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new(5);

        let ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        let popped_val = stack.pop();
        assert_eq!(popped_val, Some(542u32));
    }

    #[test]
    fn stack_push_when_full_returns_stackfullerror() {
        let mut stack = Stack::new(5);

        let mut ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(543u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(544u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(545u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(546u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(547u32);
        assert_eq!(ret, Err(StackFullError));
    }

    #[test]
    fn stack_push_pop_in_correct_order() {
        let mut stack = Stack::new(5);

        let mut ret = stack.push(542u32);
        assert_eq!(ret, Ok(()));

        ret = stack.push(543u32);
        assert_eq!(ret, Ok(()));

        let mut popped_val = stack.pop();
        assert_eq!(popped_val, Some(543u32));

        popped_val = stack.pop();
        assert_eq!(popped_val, Some(542u32));
    }

    #[test]
    fn stack_pop_when_empty_returns_none() {
        let mut stack = Stack::<u32>::new(5);

        let ret = stack.pop();
        assert_eq!(ret, None);
    }

    #[test]
    fn stack_new_returns_empty_stack() {
        let stack = Stack::<u32>::new(5);

        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn stack_new_returns_stack_with_correct_capacity() {
        let stack = Stack::<u32>::new(5);

        assert_eq!(stack.stack.capacity(), 5);
    }

    #[test]
    #[should_panic]
    fn stack_allocate_max_isize_should_panic() {
        // panic - trying to allocate too much memory
        let stack = Stack::<u32>::new(isize::MAX as usize); 
        
        assert_eq!(stack.stack.capacity(), isize::MAX as usize);
    }
}
