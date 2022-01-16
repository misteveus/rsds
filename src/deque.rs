
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct DequeFullError;

pub struct Deque<T: Clone> {
    data: Vec<Option<T>>,
    count: usize,
    tail: usize,
}

impl<T: Clone> Deque<T> {
    pub fn new(size: usize) -> Self {
        let mut d = Deque {
            data: Vec::with_capacity(size),
            count: 0,
            tail: 0,
        };

        for _ in 0..d.data.capacity() {
            d.data.push(None);
        }

        d
    }

    pub fn push_front(&mut self, val: T) -> Result<(), DequeFullError> {
        if self.count == self.data.capacity() {
            Err(DequeFullError)
        } else {
            let head = (self.tail + self.data.capacity() - self.count) % self.data.capacity();
            
            self.data[head] = Some(val);
            self.count += 1;

            Ok(())
        }
    }

    pub fn push_back(&mut self, val: T) -> Result<(), DequeFullError> {
        if self.count == self.data.capacity() {
            Err(DequeFullError)
        } else {
            self.tail = (self.tail + self.data.capacity() - 1) % self.data.capacity(); 
            self.data[self.tail] = Some(val);
            self.count += 1;

            Ok(())
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            let head = (self.tail + self.data.capacity() - self.count) % self.data.capacity();
            let ret = self.data[head].clone();
            
            self.data[head] = None;
            self.count -= 1;

            ret
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            let ret = self.data[self.tail].clone();
            
            self.data[self.tail] = None;
            self.tail = (self.tail + self.data.capacity() - 1) % self.data.capacity();
            self.count -= 1;

            ret
        }
    }

    #[must_use]
    pub fn peek_front(&self) -> &Option<T> {
        if self.count == 0 {
            &None
        } else {
            let head = (self.tail + self.data.capacity() - self.count) % self.data.capacity();
    
            self.data.get(head).unwrap()
        }
    }

    #[must_use]
    pub fn peek_back(&self) -> &Option<T> {
       if self.count == 0 {
            &None
        } else {
            self.data.get(self.tail).unwrap()
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test_deque {
    use crate::deque::*;

    #[test]
    fn deque_new_creates_correct_capcity() {
        let d = Deque::<u32>::new(5);

        assert_eq!(d.data.capacity(), 5);
    }

    #[test]
    fn deque_new_fills_inner_vec_with_none() {
        let d = Deque::<u32>::new(5);

        for element in &d.data {
            assert_eq!(element, None);
        } 
    }

    #[test]
    fn deque_new_initial_size_zero() {
        let d = Deque::<u32>::new(5);
        let size = d.size();
        
        assert_eq!(size, 0);
    }
}