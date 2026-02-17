//! Exercise 06: Generic Display - Use Display trait bound
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use Display trait bound
//! - Format generic values
//! - Work with string formatting

use std::fmt::Display;

/// Converts a value to a formatted string with a prefix.
pub fn with_prefix<T: Display>(prefix: &str, value: T) -> String {
    todo!("Implement with_prefix")
}

/// Creates a comma-separated list from a slice.
pub fn join_display<T: Display>(items: &[T]) -> String {
    todo!("Implement join_display")
}

/// A generic formatter that wraps values with decorations.
pub struct Formatter<T: Display> {
    left: String,
    right: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Display> Formatter<T> {
    /// Creates a new Formatter with left and right decorations.
    pub fn new(left: String, right: String) -> Self {
        todo!("Implement new")
    }

    /// Formats a value with decorations.
    pub fn format(&self, value: T) -> String {
        todo!("Implement format")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_prefix_integer() {
        assert_eq!(with_prefix("Number", 42), "Number: 42");
    }

    #[test]
    fn test_with_prefix_string() {
        assert_eq!(with_prefix("Name", "Alice"), "Name: Alice");
    }

    #[test]
    fn test_join_display_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(join_display(&numbers), "1, 2, 3, 4, 5");
    }

    #[test]
    fn test_join_display_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(join_display(&empty), "");
    }

    #[test]
    fn test_join_display_single() {
        let single = vec!["only"];
        assert_eq!(join_display(&single), "only");
    }

    #[test]
    fn test_formatter() {
        let formatter: Formatter<i32> = Formatter::new("[".to_string(), "]".to_string());
        assert_eq!(formatter.format(42), "[42]");
    }

    #[test]
    fn test_formatter_string() {
        let formatter: Formatter<&str> = Formatter::new("<<".to_string(), ">>".to_string());
        assert_eq!(formatter.format("hello"), "<<hello>>");
    }

    #[test]
    fn test_formatter_empty_decorations() {
        let formatter: Formatter<i32> = Formatter::new("".to_string(), "".to_string());
        assert_eq!(formatter.format(100), "100");
    }
}
