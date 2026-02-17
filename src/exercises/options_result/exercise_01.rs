//! Exercise 01: Basic Option - Return Some or None
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand the Option<T> type
//! - Use Some(value) and None variants
//! - Practice pattern matching with Option

/// Find the first positive number in a slice.
/// Returns Some(number) if found, None otherwise.
pub fn find_first_positive(numbers: &[i32]) -> Option<i32>  {
    todo!("Return Some(number) if found, None otherwise.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_positive_found() {
        assert_eq!(find_first_positive(&[1, 2, 3]), Some(1));
        assert_eq!(find_first_positive(&[-5, -3, 10, 20]), Some(10));
        assert_eq!(find_first_positive(&[0, 0, 5]), Some(5));
    }

    #[test]
    fn test_find_first_positive_not_found() {
        assert_eq!(find_first_positive(&[-1, -2, -3]), None);
        assert_eq!(find_first_positive(&[0, 0, 0]), None);
        assert_eq!(find_first_positive(&[]), None);
    }
}
