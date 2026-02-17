//! Exercise 22: Custom Smart Pointer - Implement Deref trait
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement Deref trait for custom types
//! - Understand deref coercion
//! - Create smart pointer-like types

use std::ops::Deref;

/// A simple wrapper that counts dereferences.
pub struct CountedBox<T> {
    value: Box<T>,
    deref_count: std::cell::Cell<usize>,
}

impl<T> CountedBox<T> {
    pub fn new(value: T) -> Self  {
        todo!("A simple wrapper that counts dereferences.")
    }

    pub fn deref_count(&self) -> usize  {
        todo!("Implement deref_count")
    }
}

impl<T> Deref for CountedBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target  {
        todo!("Implement deref")
    }
}

/// A selector that returns one of two values based on a flag.
pub struct Selector<T> {
    flag: bool,
    left: T,
    right: T,
}

impl<T> Selector<T> {
    pub fn new(flag: bool, left: T, right: T) -> Self  {
        todo!("A selector that returns one of two values based on a flag.")
    }

    pub fn flip(&mut self)  {
        todo!("Implement flip")
    }
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target  {
        todo!("Implement deref")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counted_box_basic() {
        let cb = CountedBox::new(42);
        assert_eq!(*cb, 42);
        assert_eq!(cb.deref_count(), 1);
    }

    #[test]
    fn test_counted_box_multiple_derefs() {
        let cb = CountedBox::new(String::from("test"));
        assert_eq!(cb.len(), 4); // deref 1
        assert_eq!(cb.as_str(), "test"); // deref 2
        assert_eq!(*cb, "test"); // deref 3
        assert_eq!(cb.deref_count(), 3);
    }

    #[test]
    fn test_counted_box_with_methods() {
        let cb = CountedBox::new(vec![1, 2, 3]);
        let _len = cb.len(); // deref for len
        let _first = cb.first(); // deref for first
        assert!(cb.deref_count() >= 2);
    }

    #[test]
    fn test_selector_left() {
        let sel = Selector::new(true, 10, 20);
        assert_eq!(*sel, 10);
    }

    #[test]
    fn test_selector_right() {
        let sel = Selector::new(false, 10, 20);
        assert_eq!(*sel, 20);
    }

    #[test]
    fn test_selector_flip() {
        let mut sel = Selector::new(true, "left", "right");
        assert_eq!(*sel, "left");
        
        sel.flip();
        assert_eq!(*sel, "right");
        
        sel.flip();
        assert_eq!(*sel, "left");
    }

    #[test]
    fn test_selector_with_strings() {
        let sel = Selector::new(
            true,
            String::from("hello"),
            String::from("world"),
        );
        assert_eq!(sel.len(), 5); // deref coercion to &str
    }

    #[test]
    fn test_deref_coercion() {
        fn take_str(s: &str) -> usize {
            s.len()
        }

        let cb = CountedBox::new(String::from("test"));
        // CountedBox<String> -> &String -> &str
        assert_eq!(take_str(&cb), 4);
    }
}
