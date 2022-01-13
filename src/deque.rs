
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct DequeFullError;

pub struct Deque<T> {
    data: Vec<T>,
    count: usize,
    tail: usize,
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {

    }

    pub fn push_front(&self, val: T) -> Result<(), DequeFullError> {

    }

    pub fn push_back(&self, val: T) -> Result<(), DequeFullError> {

    }

    pub fn pop_front(&self) -> Option<T> {

    }

    pub fn pop_back(&self) -> Option<T> {
        
    }

    pub fn peek_front(&self) -> &Option<T> {

    }

    pub fn peek_back(&self) -> &Option<T> {
        
    }

    pub fn size(&self) -> usize {

    }
}

#[cfg(test)]
mod test_deque {
    use crate::deque::*;

    #[test]
    fn test_new_creates_correct_capcity() {
        let mut d = Deque::<u32>::new(5);

        assert_eq!(d.data.capacity(), 5);
    }
}