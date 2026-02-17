//! Exercise 07: Option::unwrap_or - Provide default values
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand unwrap_or for safe unwrapping
//! - Provide default values
//! - Convert Option to concrete value

/// Get value or return a default.
pub fn get_or_default(value: Option<i32>, default: i32) -> i32 {
    todo!("Implement get_or_default")
}

/// Sum all Some values, treating None as 0.
pub fn sum_with_defaults(values: Vec<Option<i32>>) -> i32 {
    todo!("Implement sum_with_defaults")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_default() {
        assert_eq!(get_or_default(Some(10), 5), 10);
        assert_eq!(get_or_default(None, 5), 5);
    }

    #[test]
    fn test_sum_with_defaults() {
        assert_eq!(sum_with_defaults(vec![Some(1), Some(2), Some(3)]), 6);
        assert_eq!(sum_with_defaults(vec![Some(1), None, Some(3)]), 4);
        assert_eq!(sum_with_defaults(vec![None, None, None]), 0);
        assert_eq!(sum_with_defaults(vec![]), 0);
    }
}
