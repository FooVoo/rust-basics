//! Exercise 03: Generic Pair - Work with multiple generic type parameters
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use multiple type parameters
//! - Implement methods for multi-parameter generics
//! - Understand type inference with generics

/// A generic pair that can hold two values of different types.
pub struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    /// Creates a new Pair.
    pub fn new(first: T, second: U) -> Self  {
        todo!("Create a new Pair.")
    }

    /// Returns references to both values.
    pub fn get_both(&self) -> (&T, &U)  {
        todo!("Return references to both values.")
    }

    /// Returns a reference to the first value.
    pub fn first(&self) -> &T  {
        todo!("Return a reference to the first value.")
    }

    /// Returns a reference to the second value.
    pub fn second(&self) -> &U  {
        todo!("Return a reference to the second value.")
    }

    /// Swaps the pair.
    pub fn swap(self) -> Pair<U, T>  {
        todo!("Swaps the pair.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_creation() {
        let pair = Pair::new(42, "hello");
        assert_eq!(*pair.first(), 42);
        assert_eq!(*pair.second(), "hello");
    }

    #[test]
    fn test_get_both() {
        let pair = Pair::new(100, 3.14);
        let (first, second) = pair.get_both();
        assert_eq!(*first, 100);
        assert_eq!(*second, 3.14);
    }

    #[test]
    fn test_swap() {
        let pair = Pair::new("first", 123);
        let swapped = pair.swap();
        assert_eq!(*swapped.first(), 123);
        assert_eq!(*swapped.second(), "first");
    }

    #[test]
    fn test_same_types() {
        let pair = Pair::new(10, 20);
        assert_eq!(*pair.first(), 10);
        assert_eq!(*pair.second(), 20);
    }

    #[test]
    fn test_complex_types() {
        let pair = Pair::new(vec![1, 2, 3], Some("value"));
        assert_eq!(pair.first().len(), 3);
        assert_eq!(pair.second(), &Some("value"));
    }
}
