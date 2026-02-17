# Options and Result Exercises

This section contains 30 exercises focused on working with Option<T> and Result<T, E> types.

## Learning Objectives

- Master Option<T> for optional values
- Understand Result<T, E> for error handling
- Use pattern matching with Option and Result
- Work with combinators (map, and_then, etc.)
- Convert between Option and Result
- Handle None and Err cases gracefully
- Chain operations on optional values
- Unwrap safely
- Provide default values

## Topics Covered

1. **Option Basics** - Some and None variants
2. **Result Basics** - Ok and Err variants
3. **Pattern Matching** - match expressions
4. **Unwrapping** - unwrap, expect, unwrap_or
5. **Mapping** - map, map_or, map_or_else
6. **Chaining** - and_then, or_else
7. **Conversions** - ok_or, ok, err
8. **Filtering** - filter on Option
9. **Flattening** - flatten for nested Options
10. **Error Propagation** - ? operator

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic Option and Result usage
- **Medium** (Exercises 09-20): Combinators, chaining, conversions
- **Hard** (Exercises 21-28): Complex transformations, error handling
- **Expert** (Exercises 29-30): Advanced patterns, custom utilities

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test options_result::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all options_result tests
cargo test options_result

# Run a specific exercise
cargo test options_result::exercise_01

# Run with output
cargo test options_result::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of enums
- Familiarity with pattern matching

## Tips

- Prefer combinators over explicit matching when possible
- Use `?` for cleaner error propagation
- `unwrap()` should only be used when you're certain the value exists
- Use `expect()` with a message for better error messages
- `unwrap_or` and `unwrap_or_else` are safer alternatives
- Remember: Option represents optional values, Result represents operations that can fail

## Additional Resources

- [Rust Book - Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
- [Option Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Result Documentation](https://doc.rust-lang.org/std/result/enum.Result.html)
