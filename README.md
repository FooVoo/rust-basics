# Rust Basics - Comprehensive Exercise Collection

A comprehensive collection of Rust programming exercises designed to help you master the fundamentals and advanced concepts of Rust through Test-Driven Development (TDD).

## 📚 Overview

This repository contains **330 hands-on exercises** across **11 fundamental Rust themes**, ranging from beginner to expert difficulty levels. Each exercise follows TDD principles, allowing you to learn by implementing solutions to failing tests.

## 🎯 Learning Themes

The exercises are organized into the following themes:

### 1. **Error Handling** (30 exercises)
Master error handling patterns in Rust, including:
- Result and error types
- Error propagation with `?` operator
- Custom error types
- Error conversion and handling strategies

### 2. **Memory Management** (30 exercises)
Understand Rust's ownership system:
- Ownership rules and move semantics
- Borrowing and references
- Lifetimes and lifetime annotations
- Stack vs heap allocation

### 3. **Iterators** (30 exercises)
Learn iterator patterns and adapters:
- Creating and consuming iterators
- Iterator adapters (`map`, `filter`, `fold`, etc.)
- Custom iterators
- Iterator performance optimization

### 4. **Combinators** (30 exercises)
Master functional programming patterns:
- Option combinators (`map`, `and_then`, `or_else`)
- Result combinators
- Chaining operations
- Functional composition

### 5. **Async Rust** (30 exercises)
Work with asynchronous programming:
- `async`/`await` syntax
- Futures and polling
- Tokio runtime
- Concurrent async operations

### 6. **Threads** (30 exercises)
Concurrent programming and synchronization:
- Thread creation and management
- Message passing with channels
- Shared state with `Mutex` and `Arc`
- Thread safety patterns

### 7. **Polymorphism** (30 exercises)
Trait objects and dynamic dispatch:
- Trait objects (`dyn Trait`)
- Static vs dynamic dispatch
- Object safety rules
- Polymorphic patterns

### 8. **Generics** (30 exercises)
Type parameters and trait bounds:
- Generic functions and structs
- Trait bounds and where clauses
- Associated types
- Generic trait implementations

### 9. **Smart Pointers** (30 exercises)
Box, Rc, Arc, RefCell, and custom pointers:
- `Box<T>` for heap allocation
- `Rc<T>` and `Arc<T>` for shared ownership
- `RefCell<T>` for interior mutability
- Custom smart pointer implementations

### 10. **Options/Result Types** (30 exercises)
Working with Option and Result:
- Pattern matching with Option/Result
- Transforming and combining values
- Error handling best practices
- Idiomatic Rust patterns

### 11. **Enums** (30 exercises)
Algebraic data types and pattern matching:
- Basic and complex enum definitions
- Pattern matching and destructuring
- Enums with data
- Match guards and if-let

## 🚀 Getting Started

### Prerequisites

- **Rust** (1.75 or later): [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo**: Comes bundled with Rust

### Installation

1. Clone the repository:
```bash
git clone https://github.com/FooVoo/rust-basics.git
cd rust-basics
```

2. Build the project:
```bash
cargo build
```

The project compiles successfully out of the box — all exercises use `todo!()` placeholders that compile but panic at runtime, enabling a proper TDD workflow.

## 📖 How to Use

### TDD Workflow (Red-Green-Refactor)

This project follows the **Test-Driven Development** methodology. Every exercise has pre-written tests and function skeletons with `todo!()` markers. Your goal is to replace each `todo!()` with a working implementation.

1. **🔴 Red** — Pick an exercise and run its tests to see failures:
   ```bash
   cargo test enums::exercise_01
   ```
   Tests will fail with `todo!()` panic messages indicating what to implement.

2. **🟢 Green** — Implement the function to make the tests pass:
   - Open the exercise file (e.g., `src/exercises/enums/exercise_01.rs`)
   - Read the documentation, function signatures, and test cases
   - Replace the `todo!()` macro with your implementation
   - Run the tests again to verify your solution

3. **♻️ Refactor** — Improve your implementation while keeping tests green:
   - Simplify logic, remove duplication, use idiomatic Rust patterns
   - Re-run the tests to ensure nothing broke

### Running Tests

Run all tests (most will fail until you implement the exercises):
```bash
cargo test
```

Run tests for a specific theme:
```bash
cargo test enums
cargo test async_rust
cargo test error_handling
```

Run tests for a single exercise:
```bash
cargo test enums::exercise_01
```

Run tests with output:
```bash
cargo test -- --nocapture
```

### Exercise Structure

Each exercise file contains:
- **Documentation**: Learning objectives and concepts
- **Code skeleton**: Type definitions and function signatures
- **todo!() markers**: Placeholders for your implementation
- **Tests**: Comprehensive test cases to validate your solution

Example:
```rust
//! Exercise 01: Basic Enum - Days of the Week
//! Difficulty: Easy

/// Returns true if the given day is a weekend day
pub fn is_weekend(day: DayOfWeek) -> bool {
    todo!("Implement is_weekend")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekend_days() {
        assert!(is_weekend(DayOfWeek::Saturday));
        assert!(is_weekend(DayOfWeek::Sunday));
    }
}
```

## 📂 Project Structure

```
rust-basics/
├── Cargo.toml              # Project configuration and dependencies
├── README.md               # This file
├── TRANSFORMATION_SUMMARY.md # Details on the TDD transformation
├── src/
│   ├── lib.rs              # Library root — declares exercise modules
│   ├── main.rs             # Default binary entry point
│   ├── bin/
│   │   └── traits/
│   │       └── bin.rs      # Traits exploration binary (cargo run --bin traits)
│   └── exercises/          # Exercise modules (330 exercises total)
│       ├── mod.rs          # Exercise module declarations
│       ├── async_rust/     # 30 async/await exercises
│       ├── combinators/    # 30 combinator exercises
│       ├── enums/          # 30 enum exercises
│       ├── error_handling/ # 30 error handling exercises
│       ├── generics/       # 30 generic exercises
│       ├── iterators/      # 30 iterator exercises
│       ├── memory_management/ # 30 ownership/lifetime exercises
│       ├── options_result/ # 30 Option/Result exercises
│       ├── polymorphism/   # 30 polymorphism exercises
│       ├── smart_pointers/ # 30 smart pointer exercises
│       └── threads/        # 30 threading exercises
└── Cargo.lock              # Dependency lock file
```

## 🎓 Difficulty Levels

Exercises are categorized by difficulty:

- **Easy** (Beginner): Basic concepts and syntax - perfect for getting started
- **Medium** (Intermediate): Combining concepts, real-world scenarios
- **Hard** (Advanced): Complex patterns and edge cases
- **Expert** (Master): Advanced techniques and optimizations

## 📚 Official Rust Documentation Resources

### Essential Reading
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - The official Rust book
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust through annotated examples
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - The dark arts of unsafe Rust

### Reference Documentation
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) - Complete API reference
- [The Rust Reference](https://doc.rust-lang.org/reference/) - Language reference manual
- [The Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager and build system

### Specific Topics
- [Async Book](https://rust-lang.github.io/async-book/) - Asynchronous programming in Rust
- [Tokio Documentation](https://tokio.rs/tokio/tutorial) - Async runtime tutorial
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) - Error handling patterns
- [Ownership & Borrowing](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) - Memory management
- [Generics](https://doc.rust-lang.org/book/ch10-00-generics.html) - Generic types
- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) - Trait definitions and implementations
- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) - Smart pointer types
- [Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) - Fearless concurrency

### Community Resources
- [Rust Forum](https://users.rust-lang.org/) - Community discussion
- [Rust Subreddit](https://www.reddit.com/r/rust/) - r/rust community
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly Rust newsletter
- [Rust Playground](https://play.rust-lang.org/) - Online Rust compiler

## 🛠️ Dependencies

This project uses the following crates:
- **tokio** - Async runtime for Rust
- **futures** - Zero-cost async abstractions
- **async-trait** - Async trait methods
- **tokio-util** - Additional Tokio utilities

## 💡 Tips for Success

1. **Start with Easy exercises** - Build confidence with fundamental concepts
2. **Read the documentation** - Each exercise has learning objectives
3. **Run tests frequently** - Use TDD to guide your implementation
4. **Experiment** - Try different approaches and learn from compiler errors
5. **Read error messages** - Rust's compiler provides excellent guidance
6. **Consult the docs** - Use the linked resources when stuck
7. **Practice regularly** - Consistency is key to mastering Rust

## 🤝 Contributing

Contributions are welcome! If you find issues or want to add more exercises:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new exercises
5. Submit a pull request

## 📝 License

This project is open source and available for educational purposes.

## 🙏 Acknowledgments

Built with the goal of making Rust learning accessible through hands-on practice and Test-Driven Development.

---

**Happy Learning! 🦀**

Start your Rust journey by running `cargo test` and implementing your first exercise!
