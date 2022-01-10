
use std::vec::Vec;
use std::fmt;

/// `Error` type indicating the `Queue` is full.
#[derive(Debug, PartialEq)]
pub struct QueueFullError;

impl fmt::Display for QueueFullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "queue is full")
    }
}

/// A heap allocated `Queue` of type `T`. The type must 
/// implement the `Clone` trait.
pub struct Queue<T: Clone> {
    queue: Vec<Option<T>>,
    count: usize,
    tail: usize,
}


impl<T: Clone> Queue<T> {
    /// Create a new `Queue` with a max capacity of `size`. 
    /// ```
    /// use rsds::queue::Queue;
    /// 
    /// // Empty Queue created capable of holding up to 5 u32 elements. 
    /// let q = Queue::<u32>::new(5);
    /// ```
    pub fn new(size: usize) -> Self {
        let mut queue = Queue {
            queue: Vec::<Option<T>>::with_capacity(size),
            count: 0,
            tail: 0, 
        };

        for _ in 0..queue.queue.capacity() {
            queue.queue.push(None);
        }

        queue
    }

    /// Places a value at the end of the `Queue` if there is room or 
    /// return a `QueueFullError` if full.
    /// ```
    /// use rsds::queue::Queue;
    ///
    /// let mut q = Queue::<u32>::new(5);
    /// 
    /// q.enqueue(42u32);
    /// ```
    pub fn enqueue(&mut self, val: T) -> Result<(), QueueFullError> {
        if self.count == self.queue.capacity() {
            Err(QueueFullError)
        } else {
            self.queue[self.tail] = Some(val);
            self.count += 1;
            self.tail += 1;
            self.tail %= self.queue.capacity();
            Ok(())
        }
    }

    /// Removes a value from the front of the `Queue` as an `Option<T>` or `None`.
    /// if the Queue is empty.
    /// ```
    /// use rsds::queue::Queue;
    /// 
    /// let mut q = Queue::<u32>::new(5);
    /// 
    /// q.enqueue(42u32);
    /// 
    /// // Remove value from front of Queue. 
    /// let removed = q.dequeue();
    /// 
    /// assert_eq!(removed, Some(42u32));
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            
            let head = (self.tail + self.queue.capacity() - self.count) % self.queue.capacity();
            let ret = self.queue[head].clone();
            
            self.queue[head] = None;
            self.count -= 1;

            ret
        }
    }

    /// Returns an immutable reference to front of Queue.
    /// ```
    /// use rsds::queue::Queue;
    /// 
    /// // Creates an empty Queue capable of holding up to 5 u32 elements. 
    /// let mut q = Queue::<u32>::new(5);
    ///
    /// q.enqueue(42u32);
    /// 
    /// let peeked = q.peek();
    /// assert_eq!(peeked, &Some(42u32));
    /// ``` 
    pub fn peek(&self) -> &Option<T> {
        let head = (self.tail + self.queue.capacity() - self.count) % self.queue.capacity();
        self.queue.get(head).unwrap()
    }
}

#[cfg(test)]
mod test_queue {
    use crate::queue::*;

    #[test]
    fn queue_new_fills_inner_vec() {
        let queue = Queue::<u32>::new(5);

        assert_eq!(queue.queue.len(), 5);
    }

    #[test]
    fn queue_new_creates_queue_with_correct_capacity() {
        let queue = Queue::<u32>::new(5);

        assert_eq!(queue.queue.capacity(), 5);
    }

    #[test]
    fn queue_new_creates_inner_vec_filled_with_none() {
        let queue = Queue::<u32>::new(5);

        for item in &queue.queue {
            assert_eq!(item, &None);
        }

        assert_eq!(queue.queue.capacity(), 5);
    } 

    #[test]
    fn queue_enqueue_places_item_at_tail() {
        let mut queue = Queue::new(5);

        let mut ret = queue.enqueue(1u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(2u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(3u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(4u32);
        assert_eq!(ret, Ok(()));

        assert_eq!(&queue.queue[queue.tail - 1].unwrap(), &4u32);
    }

    #[test]
    fn queue_tail_and_head_equal_when_full() {
        let mut queue = Queue::new(5);

        let mut ret = queue.enqueue(1u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(2u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(3u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(4u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(4u32);
        assert_eq!(ret, Ok(()));

        let head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(head, queue.tail);
    }

    #[test]
    fn queue_count_zero_when_empty() {
        let mut queue = Queue::new(5);

        let mut ret = queue.enqueue(1u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(2u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(3u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(4u32);
        assert_eq!(ret, Ok(()));

        let mut ret = queue.dequeue();
        assert_eq!(ret, Some(1u32));

        ret = queue.dequeue();
        assert_eq!(ret, Some(2u32));

        ret = queue.dequeue();
        assert_eq!(ret, Some(3u32));

        ret = queue.dequeue();
        assert_eq!(ret, Some(4u32));

        assert_eq!(queue.count, 0);
    }

    #[test]
    fn queue_enqueue_returns_queuefullerror_when_already_full() {
        let mut queue = Queue::new(5);

        let mut ret = queue.enqueue(1u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(2u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(3u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(4u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(5u32);
        assert_eq!(ret, Ok(()));

        ret = queue.enqueue(6u32);
        assert_eq!(ret, Err(QueueFullError));
    }

    #[test]
    fn queue_peek_returns_ref_to_head() {
        let mut queue = Queue::new(5);

        let mut ret = queue.enqueue(1u32);
        assert_eq!(ret, Ok(()));

        let mut peek = queue.peek();
        let mut head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        ret = queue.enqueue(2u32);
        assert_eq!(ret, Ok(()));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        ret = queue.enqueue(3u32);
        assert_eq!(ret, Ok(()));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        ret = queue.enqueue(4u32);
        assert_eq!(ret, Ok(()));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        let mut ret = queue.dequeue();
        assert_eq!(ret, Some(1u32));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        ret = queue.dequeue();
        assert_eq!(ret, Some(2u32));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        ret = queue.dequeue();
        assert_eq!(ret, Some(3u32));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));

        ret = queue.dequeue();
        assert_eq!(ret, Some(4u32));

        peek = queue.peek();
        head = (queue.tail + queue.queue.capacity() - queue.count) % queue.queue.capacity();
        assert_eq!(Some(peek), queue.queue.get(head));
    }
}
