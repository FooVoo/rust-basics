//! Exercise 01: Basic Box - Store a value on the heap
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Box<T> as a smart pointer
//! - Learn heap allocation basics
//! - Work with boxed values

/// Create a Box containing the given value.
pub fn create_boxed_value(value: i32) -> Box<i32> {
    todo!("Implement create_boxed_value")
}

/// Double the value inside a Box.
pub fn double_boxed(b: Box<i32>) -> Box<i32> {
    todo!("Implement double_boxed")
}

/// Sum two boxed values.
pub fn sum_boxes(a: Box<i32>, b: Box<i32>) -> i32 {
    todo!("Implement sum_boxes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_boxed_value() {
        let boxed = create_boxed_value(42);
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_double_boxed() {
        let boxed = Box::new(21);
        let doubled = double_boxed(boxed);
        assert_eq!(*doubled, 42);
    }

    #[test]
    fn test_sum_boxes() {
        let a = Box::new(10);
        let b = Box::new(32);
        assert_eq!(sum_boxes(a, b), 42);
    }

    #[test]
    fn test_box_on_heap() {
        let boxed = create_boxed_value(100);
        // Box stores data on the heap
        let stack_ptr = &boxed as *const Box<i32> as usize;
        let heap_ptr = &*boxed as *const i32 as usize;
        // The pointers should be different (one on stack, one on heap)
        assert_ne!(stack_ptr, heap_ptr);
    }
}
