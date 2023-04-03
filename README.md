# smallqueue: Rust heapless queue
Bart Massey 2023 (version 0.1.0)

An implementation of a simple queue (first-in first-out)
data structure that uses no heap storage. As such, this
queue can be used in `no_std` programs, and may be more
efficient than [std::collections::VecDeque](https://doc.rust-lang.org/stable/alloc/collections/vec_deque/struct.VecDeque.html) in some
situations.

The queue capacity is specified at compile time using a
"const generic" value. Internally, the queue is implemented
using an array with a start index and a length.

This data structure is not inherently thread-safe.

## Examples

```rust
let mut q: Queue<3, usize> = Queue::default();
q.insert(17);
q.insert(18);
assert_eq!(17, q.extract().unwrap());
assert_eq!(18, q.extract().unwrap());
assert!(q.is_empty());
```

## Further Acknowledgments

Thanks to the `cargo-readme` crate for generation of this `README`.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
