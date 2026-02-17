# Error Handling Exercises

This section contains 30 exercises focused on error handling patterns in Rust.

## Learning Objectives

- Master the Result<T, E> type
- Understand error propagation with the ? operator
- Create custom error types
- Convert between error types
- Work with multiple error types
- Implement the Error trait
- Use anyhow and thiserror patterns (conceptually)
- Handle recoverable vs unrecoverable errors
- Apply error handling best practices

## Topics Covered

1. **Result Type** - Understanding Result<T, E>
2. **Error Propagation** - Using the ? operator
3. **Custom Error Types** - Defining your own errors
4. **Error Conversion** - From trait and error mapping
5. **Multiple Error Types** - Handling different error kinds
6. **Error Trait** - Implementing std::error::Error
7. **Error Context** - Adding context to errors
8. **Recoverable Errors** - Handling errors gracefully
9. **Panic vs Result** - Choosing the right approach
10. **Error Chains** - Working with error sources

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic Result usage, error propagation
- **Medium** (Exercises 09-20): Custom errors, conversions, context
- **Hard** (Exercises 21-28): Complex error types, trait implementations
- **Expert** (Exercises 29-30): Advanced error handling patterns

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test error_handling::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all error_handling tests
cargo test error_handling

# Run a specific exercise
cargo test error_handling::exercise_01

# Run with output
cargo test error_handling::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of Result and Option types
- Familiarity with enums

## Tips

- Use `?` for cleaner error propagation
- Create descriptive error messages
- Implement `From` for automatic error conversion
- Use `map_err` to transform errors
- Consider using custom error enums for multiple error types
- Document when functions can panic vs return errors

## Additional Resources

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)
