//! Exercise 02: Option::and_then - Flatten nested Options
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Option::and_then combinator
//! - Handle operations that return Option
//! - Avoid nested Option<Option<T>>

/// Divide two numbers safely, returning None if divisor is zero.
pub fn safe_divide(numerator: i32, denominator: i32) -> Option<i32> {
    todo!("Implement safe_divide")
}

/// Chain two divisions using and_then.
pub fn divide_twice(value: i32, div1: i32, div2: i32) -> Option<i32> {
    todo!("Implement divide_twice")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_twice_success() {
        assert_eq!(divide_twice(100, 5, 2), Some(10));
        assert_eq!(divide_twice(60, 3, 4), Some(5));
    }

    #[test]
    fn test_divide_twice_failure() {
        assert_eq!(divide_twice(100, 0, 2), None);
        assert_eq!(divide_twice(100, 5, 0), None);
        assert_eq!(divide_twice(100, 0, 0), None);
    }
}
