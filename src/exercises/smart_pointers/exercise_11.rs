//! Exercise 11: RefCell Basics - Interior mutability with RefCell
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand RefCell<T> for interior mutability
//! - Learn runtime borrowing rules
//! - Work with borrow() and borrow_mut()

use std::cell::RefCell;

/// Create a RefCell containing a value.
pub fn create_refcell(value: i32) -> RefCell<i32>  {
    todo!("Create a RefCell containing a value.")
}

/// Modify a value through RefCell.
pub fn modify_through_refcell(cell: &RefCell<i32>, delta: i32)  {
    todo!("Modify a value through RefCell.")
}

/// Read a value from RefCell.
pub fn read_from_refcell(cell: &RefCell<i32>) -> i32  {
    todo!("Read a value from RefCell.")
}

/// A struct with interior mutability.
pub struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    pub fn new() -> Self  {
        todo!("A struct with interior mutability.")
    }

    pub fn increment(&self)  {
        todo!("Implement increment")
    }

    pub fn get(&self) -> i32  {
        todo!("Implement get")
    }

    pub fn add(&self, n: i32)  {
        todo!("Implement add")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_refcell() {
        let cell = create_refcell(42);
        assert_eq!(*cell.borrow(), 42);
    }

    #[test]
    fn test_modify_through_refcell() {
        let cell = create_refcell(10);
        modify_through_refcell(&cell, 5);
        assert_eq!(*cell.borrow(), 15);
    }

    #[test]
    fn test_read_from_refcell() {
        let cell = create_refcell(100);
        assert_eq!(read_from_refcell(&cell), 100);
    }

    #[test]
    fn test_counter() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
        
        counter.increment();
        assert_eq!(counter.get(), 1);
        
        counter.add(5);
        assert_eq!(counter.get(), 6);
    }

    #[test]
    fn test_multiple_borrows() {
        let cell = RefCell::new(42);
        {
            let _b1 = cell.borrow();
            let _b2 = cell.borrow();
            // Multiple immutable borrows OK
        }
    }

    #[test]
    #[should_panic]
    fn test_borrow_mut_panic() {
        let cell = RefCell::new(42);
        let _b1 = cell.borrow();
        let _b2 = cell.borrow_mut(); // Should panic!
    }

    #[test]
    fn test_interior_mutability() {
        let counter = Counter::new();
        // Can mutate through immutable reference
        let _ref = &counter;
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }
}
