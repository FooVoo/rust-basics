# Combinator Exercises

This section contains 30 exercises focused on functional programming patterns and combinators in Rust.

## Learning Objectives

- Master Option and Result combinators (map, and_then, or_else)
- Understand iterator adapters and combinators
- Work with closure composition
- Chain operations functionally
- Avoid explicit pattern matching where combinators are more idiomatic
- Build complex transformations from simple combinators
- Understand the trade-offs between combinators and pattern matching

## Topics Covered

1. **Option Combinators** - map, and_then, or, or_else, filter
2. **Result Combinators** - map, map_err, and_then, or_else
3. **Iterator Combinators** - map, filter, fold, flat_map
4. **Chaining** - Composing multiple combinators
5. **Error Handling** - Using combinators for elegant error handling
6. **Default Values** - unwrap_or, unwrap_or_else, unwrap_or_default
7. **Transformation** - Converting between Option and Result
8. **Inspection** - inspect, map_or, map_or_else
9. **Custom Combinators** - Building your own combinator functions
10. **Monadic Patterns** - Understanding the functional programming concepts

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic map, filter, unwrap_or
- **Medium** (Exercises 09-20): Chaining, and_then, complex transformations
- **Hard** (Exercises 21-28): Custom combinators, advanced patterns
- **Expert** (Exercises 29-30): Monadic composition, complex functional patterns

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test combinators::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all combinator tests
cargo test combinators

# Run a specific exercise
cargo test combinators::exercise_01

# Run with output
cargo test combinators::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of Option and Result types
- Familiarity with closures
- Basic iterator knowledge

## Tips

- Try to solve problems using combinators before reaching for pattern matching
- Chain operations together for cleaner code
- Remember: `map` transforms the inner value, `and_then` chains operations that return Option/Result
- Use `filter` to keep only values that match a condition
- Use `or` for providing alternative Option/Result values

## Additional Resources

- [Rust Book - Functional Features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Option Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Result Documentation](https://doc.rust-lang.org/std/result/enum.Result.html)
