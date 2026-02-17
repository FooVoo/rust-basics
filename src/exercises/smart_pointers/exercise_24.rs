//! Exercise 24: DerefMut Implementation - Mutable dereferencing
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement DerefMut trait
//! - Understand mutable deref coercion
//! - Create mutable smart pointers

use std::ops::{Deref, DerefMut};

/// A wrapper that validates values on mutation.
pub struct ValidatedBox<T> {
    value: Box<T>,
    validator: Box<dyn Fn(&T) -> bool>,
}

impl<T> ValidatedBox<T> {
    pub fn new(value: T, validator: Box<dyn Fn(&T) -> bool>) -> Result<Self, &'static str>  {
        todo!("A wrapper that validates values on mutation.")
    }

    pub fn set(&mut self, new_value: T) -> Result<(), &'static str>  {
        todo!("Implement set")
    }
}

impl<T> Deref for ValidatedBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target  {
        todo!("Implement deref")
    }
}

impl<T> DerefMut for ValidatedBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target  {
        todo!("Implement deref_mut")
    }
}

/// A lazy box that initializes on first mutable access.
pub struct LazyBox<T> {
    value: Option<Box<T>>,
    initializer: Box<dyn Fn() -> T>,
}

impl<T> LazyBox<T> {
    pub fn new(initializer: Box<dyn Fn() -> T>) -> Self  {
        todo!("A lazy box that initializes on first mutable access.")
    }

    pub fn is_initialized(&self) -> bool  {
        todo!("Implement is_initialized")
    }

    fn ensure_initialized(&mut self)  {
        todo!("Implement ensure_initialized")
    }
}

impl<T> Deref for LazyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target  {
        todo!("Implement deref")
    }
}

impl<T> DerefMut for LazyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target  {
        todo!("Implement deref_mut")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validated_box_create() {
        let positive = |x: &i32| *x > 0;
        let vb = ValidatedBox::new(42, Box::new(positive));
        assert!(vb.is_ok());
        assert_eq!(*vb.unwrap(), 42);
    }

    #[test]
    fn test_validated_box_invalid_create() {
        let positive = |x: &i32| *x > 0;
        let vb = ValidatedBox::new(-5, Box::new(positive));
        assert!(vb.is_err());
    }

    #[test]
    fn test_validated_box_set() {
        let positive = |x: &i32| *x > 0;
        let mut vb = ValidatedBox::new(10, Box::new(positive)).unwrap();
        
        assert!(vb.set(20).is_ok());
        assert_eq!(*vb, 20);
        
        assert!(vb.set(-5).is_err());
        assert_eq!(*vb, 20); // Unchanged
    }

    #[test]
    fn test_validated_box_deref_mut() {
        let in_range = |x: &i32| *x >= 0 && *x <= 100;
        let mut vb = ValidatedBox::new(50, Box::new(in_range)).unwrap();
        
        // Direct mutation through DerefMut
        *vb = 75;
        assert_eq!(*vb, 75);
    }

    #[test]
    fn test_validated_box_with_string() {
        let not_empty = |s: &String| !s.is_empty();
        let mut vb = ValidatedBox::new("hello".to_string(), Box::new(not_empty)).unwrap();
        
        vb.push_str(" world");
        assert_eq!(*vb, "hello world");
    }

    #[test]
    fn test_lazy_box_not_initialized() {
        let lb: LazyBox<i32> = LazyBox::new(Box::new(|| 42));
        assert!(!lb.is_initialized());
    }

    #[test]
    fn test_lazy_box_initialize_on_mut() {
        let mut lb: LazyBox<i32> = LazyBox::new(Box::new(|| 42));
        *lb += 10; // Initializes and modifies
        assert!(lb.is_initialized());
        assert_eq!(*lb, 52);
    }

    #[test]
    fn test_lazy_box_with_vec() {
        let mut lb: LazyBox<Vec<i32>> = LazyBox::new(Box::new(|| vec![1, 2, 3]));
        
        lb.push(4);
        assert!(lb.is_initialized());
        assert_eq!(*lb, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_lazy_box_expensive_init() {
        let lb: LazyBox<i32> = LazyBox::new(Box::new(|| 100));
        
        assert!(!lb.is_initialized());
        // Initialization happens on first mutable access
    }
}
