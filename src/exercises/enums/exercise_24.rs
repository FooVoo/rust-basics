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
    pub fn new() -> Self {
        List::Nil
    }

    /// Adds an element to the front of the list
    pub fn cons(value: T, list: List<T>) -> Self {
        List::Cons(value, Box::new(list))
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    /// Gets the first element
    pub fn head(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(value, _) => Some(value),
        }
    }

    /// Gets the tail of the list
    pub fn tail(&self) -> Option<&List<T>> {
        match self {
            List::Nil => None,
            List::Cons(_, tail) => Some(tail),
        }
    }
}

impl<T: Clone> List<T> {
    /// Converts the list to a vector
    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self;
        while let List::Cons(value, tail) = current {
            result.push(value.clone());
            current = tail;
        }
        result
    }
}

/// Creates a list from a vector
pub fn from_vec<T>(vec: Vec<T>) -> List<T> {
    vec.into_iter()
        .rev()
        .fold(List::new(), |acc, item| List::cons(item, acc))
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
