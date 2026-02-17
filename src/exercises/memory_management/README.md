# Memory Management Exercises

This section contains 30 exercises focused on Rust's ownership, borrowing, and lifetime system.

## Learning Objectives

- Master ownership rules
- Understand borrowing and references
- Work with mutable and immutable references
- Learn lifetime annotations
- Understand move semantics
- Work with Copy and Clone traits
- Master lifetime elision rules
- Handle multiple lifetimes
- Understand lifetime bounds
- Apply RAII patterns

## Topics Covered

1. **Ownership** - Move semantics and ownership transfer
2. **Borrowing** - References and borrowing rules
3. **Mutable References** - &mut and interior mutability
4. **Lifetimes** - Explicit lifetime annotations
5. **Lifetime Elision** - When lifetimes can be omitted
6. **Multiple Lifetimes** - Working with multiple lifetime parameters
7. **Lifetime Bounds** - Constraining lifetimes
8. **Copy vs Clone** - Understanding these traits
9. **Drop Trait** - Custom cleanup with RAII
10. **Reference Cycles** - Avoiding memory leaks

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic ownership and borrowing
- **Medium** (Exercises 09-20): Lifetimes, multiple references
- **Hard** (Exercises 21-28): Complex lifetimes, custom Drop
- **Expert** (Exercises 29-30): Advanced lifetime patterns

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test memory_management::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all memory_management tests
cargo test memory_management

# Run a specific exercise
cargo test memory_management::exercise_01

# Run with output
cargo test memory_management::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust syntax
- Understanding of stack vs heap
- Familiarity with references in other languages

## Tips

- Remember: each value has exactly one owner
- Can have many immutable references OR one mutable reference
- References must always be valid
- Lifetimes describe relationships between references
- Use lifetime elision when possible
- Think about who owns data and when it's dropped

## Additional Resources

- [Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book - Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
