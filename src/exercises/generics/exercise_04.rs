//! Exercise 04: Generic Clone - Work with trait bounds
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Apply simple trait bounds
//! - Use Clone trait as a constraint
//! - Understand when trait bounds are needed

/// A generic function that duplicates a value.
pub fn duplicate<T: Clone>(value: T) -> (T, T)  {
    todo!("A generic function that duplicates a value.")
}

/// A generic struct that stores a cloneable value and provides a duplicate.
pub struct Duplicator<T: Clone> {
    original: T,
}

impl<T: Clone> Duplicator<T> {
    /// Creates a new Duplicator.
    pub fn new(value: T) -> Self  {
        todo!("Create a new Duplicator.")
    }

    /// Returns a clone of the original value.
    pub fn get_copy(&self) -> T  {
        todo!("Return a clone of the original value.")
    }

    /// Returns a reference to the original value.
    pub fn get_ref(&self) -> &T  {
        todo!("Return a reference to the original value.")
    }

    /// Creates a vector with n copies of the value.
    pub fn make_copies(&self, n: usize) -> Vec<T>  {
        todo!("Create a vector with n copies of the value.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_integer() {
        let (a, b) = duplicate(42);
        assert_eq!(a, 42);
        assert_eq!(b, 42);
    }

    #[test]
    fn test_duplicate_string() {
        let s = "hello".to_string();
        let (a, b) = duplicate(s);
        assert_eq!(a, "hello");
        assert_eq!(b, "hello");
    }

    #[test]
    fn test_duplicator_new() {
        let dup = Duplicator::new(100);
        assert_eq!(*dup.get_ref(), 100);
    }

    #[test]
    fn test_get_copy() {
        let dup = Duplicator::new("test".to_string());
        let copy = dup.get_copy();
        assert_eq!(copy, "test");
        assert_eq!(*dup.get_ref(), "test");
    }

    #[test]
    fn test_make_copies() {
        let dup = Duplicator::new(5);
        let copies = dup.make_copies(3);
        assert_eq!(copies, vec![5, 5, 5]);
    }

    #[test]
    fn test_make_copies_empty() {
        let dup = Duplicator::new("value");
        let copies = dup.make_copies(0);
        assert_eq!(copies.len(), 0);
    }
}
