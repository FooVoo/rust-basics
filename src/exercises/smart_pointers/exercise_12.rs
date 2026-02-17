//! Exercise 12: Cell Basics - Simple interior mutability with Cell
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand Cell<T> for Copy types
//! - Learn get() and set() operations
//! - Compare Cell vs RefCell

use std::cell::Cell;

/// Create a Cell containing a value.
pub fn create_cell(value: i32) -> Cell<i32> {
    Cell::new(value)
}

/// Update a value in a Cell.
pub fn update_cell(cell: &Cell<i32>, value: i32) {
    cell.set(value);
}

/// Increment a value in a Cell.
pub fn increment_cell(cell: &Cell<i32>) {
    cell.set(cell.get() + 1);
}

/// A point with interior mutability using Cell.
pub struct Point {
    x: Cell<i32>,
    y: Cell<i32>,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x: Cell::new(x),
            y: Cell::new(y),
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x.get()
    }

    pub fn get_y(&self) -> i32 {
        self.y.get()
    }

    pub fn set_x(&self, x: i32) {
        self.x.set(x);
    }

    pub fn set_y(&self, y: i32) {
        self.y.set(y);
    }

    pub fn translate(&self, dx: i32, dy: i32) {
        self.x.set(self.x.get() + dx);
        self.y.set(self.y.get() + dy);
    }

    pub fn distance_from_origin(&self) -> f64 {
        let x = self.x.get() as f64;
        let y = self.y.get() as f64;
        (x * x + y * y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cell() {
        let cell = create_cell(42);
        assert_eq!(cell.get(), 42);
    }

    #[test]
    fn test_update_cell() {
        let cell = create_cell(10);
        update_cell(&cell, 20);
        assert_eq!(cell.get(), 20);
    }

    #[test]
    fn test_increment_cell() {
        let cell = create_cell(5);
        increment_cell(&cell);
        assert_eq!(cell.get(), 6);
        increment_cell(&cell);
        assert_eq!(cell.get(), 7);
    }

    #[test]
    fn test_point_creation() {
        let p = Point::new(3, 4);
        assert_eq!(p.get_x(), 3);
        assert_eq!(p.get_y(), 4);
    }

    #[test]
    fn test_point_setters() {
        let p = Point::new(0, 0);
        p.set_x(5);
        p.set_y(10);
        assert_eq!(p.get_x(), 5);
        assert_eq!(p.get_y(), 10);
    }

    #[test]
    fn test_point_translate() {
        let p = Point::new(1, 2);
        p.translate(3, 4);
        assert_eq!(p.get_x(), 4);
        assert_eq!(p.get_y(), 6);
    }

    #[test]
    fn test_distance_from_origin() {
        let p = Point::new(3, 4);
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_interior_mutability() {
        let p = Point::new(0, 0);
        // Can modify through immutable reference
        let _ref = &p;
        p.set_x(10);
        assert_eq!(p.get_x(), 10);
    }
}
