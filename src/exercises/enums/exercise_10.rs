//! Exercise 10: Option Patterns - Working with Option<T>
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Master Option<T> patterns and methods
//! - Use map, and_then, filter on Option
//! - Chain option operations

/// Doubles the value if present
pub fn double_option(opt: Option<i32>) -> Option<i32> {
    todo!("Implement double_option")
}

/// Returns Some(value + 10) if value > 0, None otherwise
pub fn add_ten_if_positive(opt: Option<i32>) -> Option<i32> {
    todo!("Implement add_ten_if_positive")
}

/// Chains two operations: doubles the value, then adds 5
pub fn double_and_add_five(opt: Option<i32>) -> Option<i32> {
    todo!("Implement double_and_add_five")
}

/// Returns the first Some value, or None if both are None
pub fn first_some(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!("Implement first_some")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_option() {
        assert_eq!(double_option(Some(5)), Some(10));
        assert_eq!(double_option(Some(-3)), Some(-6));
        assert_eq!(double_option(None), None);
    }

    #[test]
    fn test_add_ten_if_positive() {
        assert_eq!(add_ten_if_positive(Some(5)), Some(15));
        assert_eq!(add_ten_if_positive(Some(-5)), None);
        assert_eq!(add_ten_if_positive(Some(0)), None);
        assert_eq!(add_ten_if_positive(None), None);
    }

    #[test]
    fn test_double_and_add_five() {
        assert_eq!(double_and_add_five(Some(10)), Some(25));
        assert_eq!(double_and_add_five(Some(0)), Some(5));
        assert_eq!(double_and_add_five(None), None);
    }

    #[test]
    fn test_first_some() {
        assert_eq!(first_some(Some(1), Some(2)), Some(1));
        assert_eq!(first_some(None, Some(2)), Some(2));
        assert_eq!(first_some(Some(1), None), Some(1));
        assert_eq!(first_some(None, None), None);
    }
}
