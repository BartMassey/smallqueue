/// Check for memory leaks by creating a queue containing
/// memory references and dropping it before freeing. Run
/// this "example" with `valgrind` to verify that it is OK
/// and leaves the heap empty on exit.

use smallqueue::Queue;

fn main() {
    let mut q: Queue<4, String> = Queue::default();
    for s in ["x", "y", "z"] {
        q.insert(s.to_string()).unwrap();
    }
    drop(q);
}
