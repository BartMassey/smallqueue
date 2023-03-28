/*!
An implementation of a simple queue (first-in first-out)
data structure that uses no heap storage. As such, this
queue can be used in `no_std` programs, and may be more
efficient that [std::collections::VecDeque] in some
situations.

The queue capacity is specified at compile time using a
"const generic" value. Internally, the queue is implemented
using an array with a start index and a length.

This data structure is not inherently thread-safe.

# Examples

```
# use smallqueue::Queue;
let mut q: Queue<3, usize> = Queue::default();
q.insert(17);
q.insert(18);
assert_eq!(17, q.extract().unwrap());
assert_eq!(18, q.extract().unwrap());
assert!(q.is_empty());
```
*/

use core::{
    array,
    mem::MaybeUninit,
    ptr,
};

use thiserror::Error;

/// Queue errors.
#[derive(Debug, Error)]
pub enum QueueError {
    /// An attempt was made to insert a value into a queue
    /// that was already full.
    #[error("queue capacity exceeded")]
    Overflow,
}

/// A queue (first-in first-out) data structure of fixed
/// capacity, using no heap storage.
pub struct Queue<const C: usize, T> {
    values: [MaybeUninit<T>; C],
    start: usize,
    len: usize,
}

impl<const C: usize, T> Queue<C, T> {
    /// Insert the given `value` into the queue.
    ///
    /// See the module documentation for an example.
    ///
    /// # Errors
    ///
    /// Returns [QueueError::Overflow] if the queue is full.
    pub fn insert(&mut self, value: T) -> Result<(), QueueError> {
        let cap = self.values.len();
        if self.len + 1 > cap {
            return Err(QueueError::Overflow);
        }
        // Safety: We are only writing to a location at an index that
        // is bounds-checked.
        unsafe {
            self.values[(self.start + self.len) % cap].as_mut_ptr().write(value);
        }
        self.len += 1;
        Ok(())
    }

    /// Returns `Some` first value in the queue if one
    /// exists, and `None` otherwise.
    ///
    /// See the module documentation for an example.
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

    /// Returns the capacity of this queue (maximum number
    /// of values that may be stored) as defined at
    /// compile-time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smallqueue::Queue;
    /// let mut q: Queue<3, usize> = Queue::default();
    /// assert_eq!(3, q.capacity());
    /// ```
    pub const fn capacity(&self) -> usize {
        self.values.len()
    }

    /// Returns the number of values currently stored in the
    /// queue.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smallqueue::Queue;
    /// let mut q: Queue<3, usize> = Queue::default();
    /// q.insert(17);
    /// assert_eq!(1, q.len());
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `false` if the queue contains values, but
    /// `true` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smallqueue::Queue;
    /// let mut q: Queue<3, usize> = Queue::default();
    /// assert!(q.is_empty());
    /// q.insert(17);
    /// assert!(!q.is_empty());
    /// ```
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
