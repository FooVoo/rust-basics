//! Exercise 24: Linked List - Recursive Enum with Option
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement linked list with recursive enum
//! - Work with Option and Box together
//! - Understand memory layout of recursive types

#[derive(Debug, PartialEq, Clone)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    /// Creates an empty list
    pub fn new() -> Self  {
        todo!("Create an empty list")
    }

    /// Adds an element to the front of the list
    pub fn cons(value: T, list: List<T>) -> Self  {
        todo!("Add an element to the front of the list")
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize  {
        todo!("Return the length of the list")
    }

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool  {
        todo!("Return true if the list is empty")
    }

    /// Gets the first element
    pub fn head(&self) -> Option<&T>  {
        todo!("Get the first element")
    }

    /// Gets the tail of the list
    pub fn tail(&self) -> Option<&List<T>>  {
        todo!("Get the tail of the list")
    }
}

impl<T: Clone> List<T> {
    /// Converts the list to a vector
    pub fn to_vec(&self) -> Vec<T>  {
        todo!("Convert the list to a vector")
    }
}

/// Creates a list from a vector
pub fn from_vec<T>(vec: Vec<T>) -> List<T>  {
    todo!("Create a list from a vector")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: List<i32> = List::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_cons() {
        let list = List::cons(1, List::cons(2, List::cons(3, List::new())));
        assert_eq!(list.len(), 3);
        assert_eq!(list.head(), Some(&1));
    }

    #[test]
    fn test_tail() {
        let list = List::cons(1, List::cons(2, List::cons(3, List::new())));
        let tail = list.tail().unwrap();
        assert_eq!(tail.head(), Some(&2));
        assert_eq!(tail.len(), 2);
    }

    #[test]
    fn test_to_vec() {
        let list = List::cons(1, List::cons(2, List::cons(3, List::new())));
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_from_vec() {
        let list = from_vec(vec![1, 2, 3]);
        assert_eq!(list.len(), 3);
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }
}
