# Generics Exercises

This section contains 30 exercises focused on generic programming in Rust.

## Learning Objectives

- Understand generic function syntax
- Use type parameters in structs and enums
- Master trait bounds and where clauses
- Work with multiple type parameters
- Understand lifetime parameters with generics
- Apply associated types
- Use phantom types
- Implement generic traits
- Master higher-rank trait bounds (HRTBs)

## Topics Covered

1. **Generic Functions** - Type parameters in functions
2. **Generic Structs** - Parameterized data structures
3. **Trait Bounds** - Constraining generic types
4. **Where Clauses** - Complex trait bounds
5. **Multiple Parameters** - Working with multiple generics
6. **Lifetimes** - Combining lifetimes with generics
7. **Associated Types** - Trait-associated type parameters
8. **Phantom Types** - Zero-sized type parameters
9. **Generic Traits** - Implementing traits generically
10. **HRTBs** - Higher-rank trait bounds with for<'a>

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic generic functions and structs
- **Medium** (Exercises 09-20): Trait bounds, multiple parameters
- **Hard** (Exercises 21-28): Associated types, complex bounds
- **Expert** (Exercises 29-30): HRTBs, advanced generic patterns

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test generics::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all generics tests
cargo test generics

# Run a specific exercise
cargo test generics::exercise_01

# Run with output
cargo test generics::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of traits
- Familiarity with ownership and borrowing

## Tips

- Start simple and add trait bounds as needed
- Use `where` clauses for complex bounds
- Remember: generics are monomorphized at compile time
- Use `impl Trait` for simpler function signatures when appropriate
- Combine lifetimes and generics carefully

## Additional Resources

- [Rust Book - Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Reference - Generics](https://doc.rust-lang.org/reference/items/generics.html)
