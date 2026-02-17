//! Exercise 03: If Let - Simplified Pattern Matching
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use if let for single pattern matching
//! - Understand when to use if let vs match
//! - Work with Option enum

/// Extracts the value from Some, returns 0 for None
pub fn unwrap_or_zero(opt: Option<i32>) -> i32 {
    if let Some(value) = opt {
        value
    } else {
        0
    }
}

/// Returns true if the option contains a value greater than 10
pub fn is_greater_than_ten(opt: Option<i32>) -> bool {
    if let Some(value) = opt {
        value > 10
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_or_zero() {
        assert_eq!(unwrap_or_zero(Some(42)), 42);
        assert_eq!(unwrap_or_zero(Some(-5)), -5);
        assert_eq!(unwrap_or_zero(None), 0);
    }

    #[test]
    fn test_is_greater_than_ten() {
        assert!(is_greater_than_ten(Some(15)));
        assert!(is_greater_than_ten(Some(11)));
        assert!(!is_greater_than_ten(Some(10)));
        assert!(!is_greater_than_ten(Some(5)));
        assert!(!is_greater_than_ten(None));
    }
}
