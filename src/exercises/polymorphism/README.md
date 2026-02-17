# Polymorphism Exercises

This section contains 30 exercises focused on polymorphism and trait objects in Rust.

## Learning Objectives

- Understand trait objects and dynamic dispatch
- Work with dyn Trait syntax
- Master trait implementation
- Use trait bounds effectively
- Understand static vs dynamic dispatch
- Work with object safety rules
- Implement polymorphic designs
- Use Box<dyn Trait> and &dyn Trait
- Understand vtables and performance implications
- Apply visitor and strategy patterns

## Topics Covered

1. **Trait Objects** - dyn Trait and dynamic dispatch
2. **Object Safety** - Rules for trait objects
3. **Box<dyn Trait>** - Owned trait objects
4. **&dyn Trait** - Borrowed trait objects
5. **Multiple Traits** - Trait object with multiple bounds
6. **Static Dispatch** - impl Trait and monomorphization
7. **Dynamic Dispatch** - Runtime polymorphism
8. **Design Patterns** - Strategy, visitor, command
9. **Downcasting** - Converting trait objects to concrete types
10. **Performance** - Understanding the costs

## Difficulty Distribution

- **Easy** (Exercises 01-08): Basic traits and implementations
- **Medium** (Exercises 09-20): Trait objects, dynamic dispatch
- **Hard** (Exercises 21-28): Complex polymorphic designs
- **Expert** (Exercises 29-30): Advanced patterns, optimization

## How to Work Through These Exercises

1. **Read the exercise description** - Each exercise file starts with learning objectives
2. **Study the tests** - Tests define the expected behavior (TDD approach)
3. **Implement the solution** - Replace `todo!()` with your implementation
4. **Run the tests** - Use `cargo test polymorphism::exercise_XX` to test your solution
5. **Iterate** - If tests fail, review and refine your implementation

## Running the Exercises

```bash
# Run all polymorphism tests
cargo test polymorphism

# Run a specific exercise
cargo test polymorphism::exercise_01

# Run with output
cargo test polymorphism::exercise_01 -- --nocapture
```

## Prerequisites

- Basic Rust knowledge
- Understanding of traits
- Familiarity with generics
- Knowledge of ownership and borrowing

## Tips

- Use trait objects when you need runtime polymorphism
- Use generics when you know types at compile time
- Remember: not all traits are object-safe
- `Box<dyn Trait>` for owned, `&dyn Trait` for borrowed
- Trait objects have a small runtime cost
- Consider static dispatch (generics) for performance-critical code

## Additional Resources

- [Rust Book - Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Object Safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
- [Dynamic Dispatch](https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/)
