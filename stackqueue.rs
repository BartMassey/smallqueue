use core::{
    array,
    mem::MaybeUninit,
    ptr,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("queue capacity exceeded")]
    Overflow,
}

pub struct Queue<const C: usize, T> {
    values: [MaybeUninit<T>; C],
    start: usize,
    len: usize,
}

impl<const C: usize, T> Queue<C, T> {
    pub fn insert(&mut self, val: T) -> Result<(), QueueError> {
        let cap = self.values.len();
        if self.len + 1 > cap {
            return Err(QueueError::Overflow);
        }
        // Safety: We are only writing to a location at an index that
        // is bounds-checked.
        unsafe {
            *self.values[(self.start + self.len) % cap].as_mut_ptr() = val;
        }
        self.len += 1;
        Ok(())
    }

    pub fn extract(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let cap = self.values.len();
        // Safety: We are retrieving a value that was previously
        // inserted, as evidenced by the values of start and len.
        let val = unsafe {
            ptr::read(self.values[self.start].as_ptr())
        };
        self.start = (self.start + 1) % cap;
        self.len -= 1;
        Some(val)
    }

    pub fn capacity(&self) -> usize {
        self.values.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<const C: usize, T> Default for Queue<C, T> {
    fn default() -> Self {
        Self {
            values: array::from_fn(|_| MaybeUninit::uninit()),
            start: 0,
            len: 0,
        }
    }
}

impl<const C: usize, T> Drop for Queue<C, T> {
    fn drop(&mut self) {
        let cap = self.values.len();
        let start = self.start;
        for i in 0..self.len {
            // Safety: All of the dropped values are initialized.
            unsafe {
                ptr::drop_in_place(self.values[(start + i) % cap].as_mut_ptr());
            }
        }
    }
}

#[test]
fn test_queue() {
    #[derive(Debug, PartialEq, Eq)]
    struct S(usize);

    let mut q: Queue<3, S> = Queue::default();
    assert!(q.is_empty());
    for i in 0..3 {
        q.insert(S(i)).unwrap();
    }
    assert_eq!(3, q.len());
    let ovf = q.insert(S(3));
    assert!(matches!(ovf, Err(QueueError::Overflow)));
    assert!(matches!(q.extract(), Some(S(0))));
    q.insert(S(3)).unwrap();
    assert_eq!(3, q.len());
    for i in 1..=3 {
        assert_eq!(Some(S(i)), q.extract());
    }
    assert!(matches!(q.extract(), None));
    assert!(q.is_empty());
}
