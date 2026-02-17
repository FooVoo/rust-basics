//! Exercise 02: Recursive Types - Build a linked list using Box
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use Box<T> to enable recursive types
//! - Understand why Box is needed for recursive structures
//! - Build basic linked list operations

#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// Create an empty list.
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Prepend a value to the list.
    pub fn prepend(self, value: i32) -> Self {
        todo!("Implement prepend")
    }

    /// Get the length of the list.
    pub fn len(&self) -> usize {
        todo!("Implement len")
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        todo!("Implement is_empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = List::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_prepend() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_list_structure() {
        let list = List::new().prepend(1).prepend(2);
        match list {
            List::Cons(val, tail) => {
                assert_eq!(val, 2);
                match *tail {
                    List::Cons(val2, _) => assert_eq!(val2, 1),
                    _ => panic!("Expected Cons"),
                }
            }
            _ => panic!("Expected Cons"),
        }
    }
}
