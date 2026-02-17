//! Exercise 05: Generic Comparison - Use PartialEq trait bound
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use PartialEq trait bound
//! - Compare generic values
//! - Return boolean results from generic functions

/// Checks if two values are equal.
pub fn are_equal<T: PartialEq>(a: T, b: T) -> bool  {
    todo!("Check if two values are equal.")
}

/// Finds if a value exists in a slice.
pub fn contains<T: PartialEq>(slice: &[T], value: &T) -> bool  {
    todo!("Finds if a value exists in a slice.")
}

/// A generic struct that checks for equality.
pub struct EqualityChecker<T: PartialEq> {
    reference: T,
}

impl<T: PartialEq> EqualityChecker<T> {
    /// Creates a new EqualityChecker with a reference value.
    pub fn new(reference: T) -> Self  {
        todo!("Create a new EqualityChecker with a reference value.")
    }

    /// Checks if the given value equals the reference.
    pub fn is_equal(&self, value: &T) -> bool  {
        todo!("Check if the given value equals the reference.")
    }

    /// Counts how many items in the slice equal the reference.
    pub fn count_equal(&self, slice: &[T]) -> usize  {
        todo!("Counts how many items in the slice equal the reference.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_equal_integers() {
        assert!(are_equal(5, 5));
        assert!(!are_equal(5, 10));
    }

    #[test]
    fn test_are_equal_strings() {
        assert!(are_equal("hello".to_string(), "hello".to_string()));
        assert!(!are_equal("hello".to_string(), "world".to_string()));
    }

    #[test]
    fn test_contains_present() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(contains(&numbers, &3));
    }

    #[test]
    fn test_contains_absent() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(!contains(&numbers, &10));
    }

    #[test]
    fn test_equality_checker() {
        let checker = EqualityChecker::new(42);
        assert!(checker.is_equal(&42));
        assert!(!checker.is_equal(&43));
    }

    #[test]
    fn test_count_equal() {
        let checker = EqualityChecker::new(5);
        let numbers = vec![5, 3, 5, 7, 5, 2];
        assert_eq!(checker.count_equal(&numbers), 3);
    }

    #[test]
    fn test_count_equal_none() {
        let checker = EqualityChecker::new("missing");
        let items = vec!["a", "b", "c"];
        assert_eq!(checker.count_equal(&items), 0);
    }
}
