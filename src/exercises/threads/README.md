# Thread and Concurrency Exercises

This section contains 30 exercises focused on concurrent programming with threads in Rust.

## Learning Objectives

- Understand thread creation and joining
- Work with thread-safe data sharing
- Use Mutex and RwLock for synchronization
- Master message passing with channels
- Understand Send and Sync traits
- Work with atomic operations
- Handle thread panics
- Use thread pools
- Implement concurrent data structures
- Apply concurrency patterns safely

## Topics Covered

1. **Thread Basics** - spawn, join, thread handles
2. **Data Sharing** - Arc and thread-safe sharing
3. **Mutex** - Mutual exclusion locks
4. **RwLock** - Reader-writer locks
5. **Channels** - mpsc for message passing
6. **Send and Sync** - Thread safety markers
7. **Atomics** - Lock-free operations
8. **Barriers** - Synchronizing thread groups
9. **Scoped Threads** - Borrowing in threads
10. **Thread Pools** - Reusable thread workers

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic thread creation, joining
- **Medium** (Exercises 09-20): Mutex, channels, data sharing
- **Hard** (Exercises 21-28): Complex synchronization, atomics
- **Expert** (Exercises 29-30): Custom concurrent data structures

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test threads::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all thread tests
cargo test threads

# Run a specific exercise
cargo test threads::exercise_01

# Run with output
cargo test threads::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of ownership and borrowing
- Familiarity with Arc and Rc
- Basic concurrency concepts

## Tips

- Always join threads to ensure they complete
- Use `Arc<Mutex<T>>` for shared mutable state
- Prefer message passing (channels) over shared state when possible
- `RwLock` allows multiple readers or one writer
- Be careful of deadlocks with multiple locks
- Use scoped threads when you need to borrow data
- Atomics are for simple lock-free operations

## Additional Resources

- [Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Thread Documentation](https://doc.rust-lang.org/std/thread/)
- [Mutex Documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Channel Documentation](https://doc.rust-lang.org/std/sync/mpsc/)
