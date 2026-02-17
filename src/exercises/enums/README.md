# Enum Exercises

This section contains 30 exercises focused on enums and pattern matching in Rust.

## Learning Objectives

- Understand enum types and their variants
- Master pattern matching with match and if let
- Work with enums containing data (tuple and struct variants)
- Implement methods on enums
- Use Option and Result patterns effectively
- Create complex enum types for state machines
- Understand enum memory layout and optimizations
- Apply type-state patterns with enums

## Topics Covered

1. **Basic Enums** - Simple enum variants without data
2. **Enums with Data** - Tuple and struct variants
3. **Pattern Matching** - match, if let, while let expressions
4. **Enum Methods** - Implementing methods on enums
5. **Option Type** - Working with Option<T>
6. **Result Type** - Error handling with Result<T, E>
7. **State Machines** - Using enums to model state
8. **Recursive Enums** - Tree structures and linked lists
9. **Niche Optimization** - Memory layout optimizations
10. **Type-State Pattern** - Compile-time state enforcement

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic enums and simple pattern matching
- **Medium** (Exercises 09-20): Enums with data, methods, state machines
- **Hard** (Exercises 21-28): Recursive enums, complex patterns
- **Expert** (Exercises 29-30): Type-state patterns, advanced optimizations

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test enums::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all enum tests
cargo test enums

# Run a specific exercise
cargo test enums::exercise_01

# Run with output
cargo test enums::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust syntax
- Understanding of structs
- Basic pattern matching knowledge

## Tips

- Use `match` when you need to handle all variants
- Use `if let` when you only care about one variant
- Remember to derive useful traits like `Debug`, `PartialEq`, `Clone`
- Consider using `#[non_exhaustive]` for library enums
- Use pattern guards (`if` conditions) when you need extra logic in match arms

## Additional Resources

- [Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust Reference - Enums](https://doc.rust-lang.org/reference/items/enumerations.html)
