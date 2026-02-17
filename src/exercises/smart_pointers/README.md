# Smart Pointers Exercises

This section contains 30 exercises focused on smart pointers in Rust.

## Learning Objectives

- Understand Box<T> for heap allocation
- Master Rc<T> for reference counting
- Work with Arc<T> for thread-safe reference counting
- Use RefCell<T> for interior mutability
- Combine Rc<RefCell<T>> patterns
- Understand Cow<T> for clone-on-write
- Implement custom smart pointers
- Understand Deref and Drop traits
- Work with weak references
- Avoid reference cycles

## Topics Covered

1. **Box<T>** - Heap allocation and owned pointers
2. **Rc<T>** - Single-threaded reference counting
3. **Arc<T>** - Thread-safe reference counting
4. **RefCell<T>** - Runtime-checked borrowing
5. **Interior Mutability** - Mutating through immutable references
6. **Weak<T>** - Breaking reference cycles
7. **Cow<T>** - Clone-on-write smart pointer
8. **Deref Trait** - Custom dereference behavior
9. **Drop Trait** - Custom cleanup logic
10. **Custom Pointers** - Building your own smart pointers

## Difficulty Distribution

- **Easy** (Exercises 01-08): Box, basic Rc and Arc
- **Medium** (Exercises 09-20): RefCell, combinations, interior mutability
- **Hard** (Exercises 21-28): Weak references, custom smart pointers
- **Expert** (Exercises 29-30): Advanced patterns, custom implementations

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test smart_pointers::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all smart_pointers tests
cargo test smart_pointers

# Run a specific exercise
cargo test smart_pointers::exercise_01

# Run with output
cargo test smart_pointers::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of ownership and borrowing
- Familiarity with traits
- Knowledge of heap vs stack

## Tips

- Use `Box` when you need heap allocation or trait objects
- Use `Rc` when you need multiple ownership (single-threaded)
- Use `Arc` when you need multiple ownership (multi-threaded)
- Use `RefCell` when you need interior mutability
- Combine `Rc<RefCell<T>>` for shared mutable state
- Be careful of reference cycles with `Rc` - use `Weak` to break them
- `Cow` is great for optimization when cloning is expensive

## Additional Resources

- [Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Box Documentation](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [Rc Documentation](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [RefCell Documentation](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
