# Iterator Exercises

This section contains 30 exercises focused on iterators and iterator patterns in Rust.

## Learning Objectives

- Understand the Iterator trait
- Master iterator adapters (map, filter, fold, etc.)
- Create custom iterators
- Work with consuming and borrowing iterators
- Use iterator combinators effectively
- Understand lazy evaluation
- Implement iterator trait for custom types
- Chain complex iterator operations
- Work with infinite iterators
- Optimize with iterator methods

## Topics Covered

1. **Iterator Basics** - for loops and .iter()
2. **Iterator Adapters** - map, filter, take, skip
3. **Consuming Iterators** - collect, fold, reduce
4. **Custom Iterators** - Implementing Iterator trait
5. **Iterator Chains** - Composing multiple operations
6. **Borrowing vs Consuming** - iter, iter_mut, into_iter
7. **Infinite Iterators** - Working with endless sequences
8. **Peekable Iterators** - Looking ahead
9. **Iterator Extensions** - Custom iterator methods
10. **Performance** - Zero-cost abstractions

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic iterator usage, simple adapters
- **Medium** (Exercises 09-20): Chaining, custom iterators
- **Hard** (Exercises 21-28): Complex custom iterators, advanced patterns
- **Expert** (Exercises 29-30): Iterator trait extensions, optimization

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test iterators::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all iterator tests
cargo test iterators

# Run a specific exercise
cargo test iterators::exercise_01

# Run with output
cargo test iterators::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of closures
- Familiarity with common collections

## Tips

- Iterators are lazy - they don't do anything until consumed
- Prefer iterators over manual loops for cleaner code
- Use `collect()` to turn iterators into collections
- Chain operations for more expressive code
- Remember: `map` is lazy, `for_each` is eager

## Additional Resources

- [Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Iterator Patterns](https://rust-unofficial.github.io/patterns/patterns/behavioural/iterator.html)
