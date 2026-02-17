//! Exercise 02: Generic Struct - Define a struct with generic type parameters
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Define generic structs
//! - Implement methods for generic types
//! - Use type parameters in struct fields

/// A generic container that holds a single value.
pub struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    /// Creates a new Container with the given value.
    pub fn new(value: T) -> Self {
        todo!("Implement new")
    }

    /// Returns a reference to the contained value.
    pub fn get(&self) -> &T {
        todo!("Implement get")
    }

    /// Consumes the container and returns the value.
    pub fn into_inner(self) -> T {
        todo!("Implement into_inner")
    }

    /// Replaces the value and returns the old value.
    pub fn replace(&mut self, value: T) -> T {
        todo!("Implement replace")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_new() {
        let container = Container::new(42);
        assert_eq!(*container.get(), 42);
    }

    #[test]
    fn test_container_string() {
        let container = Container::new("hello".to_string());
        assert_eq!(container.get(), "hello");
    }

    #[test]
    fn test_into_inner() {
        let container = Container::new(vec![1, 2, 3]);
        let vec = container.into_inner();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_replace() {
        let mut container = Container::new(10);
        let old = container.replace(20);
        assert_eq!(old, 10);
        assert_eq!(*container.get(), 20);
    }

    #[test]
    fn test_multiple_types() {
        let int_container = Container::new(100);
        let string_container = Container::new("test".to_string());
        assert_eq!(*int_container.get(), 100);
        assert_eq!(string_container.get(), "test");
    }
}
