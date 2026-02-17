# Async Rust Exercises

This section contains 30 exercises focused on asynchronous programming in Rust using async/await syntax and the Tokio runtime.

## Learning Objectives

- Understand async/await syntax and semantics
- Work with Futures and async functions
- Master concurrent task execution with Tokio
- Handle async errors and timeouts
- Work with async streams and channels
- Implement async trait methods
- Understand async runtime mechanics
- Apply select! macro for concurrent operations
- Work with async I/O operations
- Master async synchronization primitives

## Topics Covered

1. **Basic Async Functions** - Simple async operations
2. **Awaiting Futures** - Chaining async operations
3. **Tokio Runtime** - Task spawning and execution
4. **Async Error Handling** - Result and error propagation
5. **Timeouts and Delays** - Working with time in async code
6. **Channels** - Async message passing
7. **Select Macro** - Concurrent awaiting
8. **Async Traits** - Using async-trait crate
9. **Streams** - Async iterators
10. **Synchronization** - Mutex, RwLock, Semaphore

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic async/await syntax
- **Medium** (Exercises 09-20): Task spawning, channels, error handling
- **Hard** (Exercises 21-28): Complex concurrent patterns, streams
- **Expert** (Exercises 29-30): Advanced async patterns, custom futures

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test async_rust::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all async_rust tests
cargo test async_rust

# Run a specific exercise
cargo test async_rust::exercise_01

# Run with output
cargo test async_rust::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge (ownership, borrowing, traits)
- Understanding of concurrency concepts
- Familiarity with the Result type

## Additional Resources

- [The Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Rust](https://rust-lang.github.io/async-book/)
