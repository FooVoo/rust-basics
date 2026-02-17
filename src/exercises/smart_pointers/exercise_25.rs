//! Exercise 25: Pin and Unpin - Understanding pinned pointers
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand Pin<T> for immovable types
//! - Learn about Unpin marker trait
//! - Work with pinned memory

use std::marker::PhantomPinned;
use std::pin::Pin;

/// A self-referential struct (needs Pin to be safe).
pub struct SelfReferential {
    data: String,
    pointer: *const String,
    _pin: PhantomPinned,
}

impl SelfReferential {
    pub fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::new(SelfReferential {
            data,
            pointer: std::ptr::null(),
            _pin: PhantomPinned,
        });

        let ptr: *const String = &boxed.data;
        boxed.pointer = ptr;

        Box::into_pin(boxed)
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn pointer_is_valid(&self) -> bool {
        !self.pointer.is_null() && self.pointer == &self.data as *const String
    }
}

/// A simple pinned counter.
pub struct PinnedCounter {
    count: i32,
    _pin: PhantomPinned,
}

impl PinnedCounter {
    pub fn new(initial: i32) -> Pin<Box<Self>> {
        Box::pin(PinnedCounter {
            count: initial,
            _pin: PhantomPinned,
        })
    }

    pub fn get(&self) -> i32 {
        self.count
    }

    pub fn increment(self: Pin<&mut Self>) {
        unsafe {
            let this = self.get_unchecked_mut();
            this.count += 1;
        }
    }
}

/// A normal (Unpin) smart pointer wrapper.
pub struct UnpinBox<T> {
    value: Box<T>,
}

impl<T> UnpinBox<T> {
    pub fn new(value: T) -> Self {
        UnpinBox {
            value: Box::new(value),
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn into_inner(self) -> T {
        *self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_referential() {
        let sr = SelfReferential::new("test".to_string());
        assert_eq!(sr.get_data(), "test");
        assert!(sr.pointer_is_valid());
    }

    #[test]
    fn test_self_referential_pin() {
        let sr = SelfReferential::new("data".to_string());
        
        // Cannot move out of Pin
        // let moved = *sr; // This would not compile
        
        // But can access through Pin
        assert_eq!(sr.as_ref().get_data(), "data");
    }

    #[test]
    fn test_pinned_counter() {
        let mut counter = PinnedCounter::new(0);
        assert_eq!(counter.as_ref().get(), 0);
        
        counter.as_mut().increment();
        assert_eq!(counter.as_ref().get(), 1);
        
        counter.as_mut().increment();
        assert_eq!(counter.as_ref().get(), 2);
    }

    #[test]
    fn test_unpin_box() {
        let mut ub = UnpinBox::new(42);
        assert_eq!(*ub.get(), 42);
        
        *ub.get_mut() = 100;
        assert_eq!(*ub.get(), 100);
    }

    #[test]
    fn test_unpin_box_into_inner() {
        let ub = UnpinBox::new(vec![1, 2, 3]);
        let vec = ub.into_inner();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_pin_box() {
        let value = Box::new(42);
        let pinned = Box::into_pin(value);
        assert_eq!(*pinned, 42);
    }

    #[test]
    fn test_unpin_types() {
        // Most types are Unpin
        fn assert_unpin<T: Unpin>() {}
        
        assert_unpin::<i32>();
        assert_unpin::<String>();
        assert_unpin::<Vec<i32>>();
        assert_unpin::<Box<i32>>();
    }

    #[test]
    fn test_pin_projection() {
        let mut counter = PinnedCounter::new(10);
        let count_before = counter.as_ref().get();
        counter.as_mut().increment();
        assert_eq!(counter.as_ref().get(), count_before + 1);
    }
}
