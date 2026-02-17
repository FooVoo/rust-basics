//! Exercise 07: Find and Position - Searching operations
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use find() to search for elements
//! - Use position() to find indices
//! - Work with any() and all()

/// Find the first even number.
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    todo!("Implement find_first_even")
}

/// Find the position of the first negative number.
pub fn find_negative_position(numbers: &[i32]) -> Option<usize> {
    todo!("Implement find_negative_position")
}

/// Check if all numbers are positive.
pub fn all_positive(numbers: &[i32]) -> bool {
    todo!("Implement all_positive")
}

/// Check if any string contains a target character.
pub fn any_contains_char(strings: &[&str], target: char) -> bool {
    todo!("Implement any_contains_char")
}

/// Find the first string longer than the given length.
pub fn find_long_string(strings: &[&str], min_length: usize) -> Option<String> {
    todo!("Implement find_long_string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_even() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
        assert_eq!(find_first_even(&[]), None);
        assert_eq!(find_first_even(&[2, 4, 6]), Some(2));
    }

    #[test]
    fn test_find_negative_position() {
        assert_eq!(find_negative_position(&[1, 2, -3, 4]), Some(2));
        assert_eq!(find_negative_position(&[1, 2, 3]), None);
        assert_eq!(find_negative_position(&[]), None);
        assert_eq!(find_negative_position(&[-1, 2, 3]), Some(0));
    }

    #[test]
    fn test_all_positive() {
        assert!(all_positive(&[1, 2, 3, 4]));
        assert!(!all_positive(&[1, -2, 3]));
        assert!(all_positive(&[]));
        assert!(!all_positive(&[0, 1, 2]));
    }

    #[test]
    fn test_any_contains_char() {
        assert!(any_contains_char(&["hello", "world"], 'o'));
        assert!(!any_contains_char(&["hello", "hi"], 'z'));
        assert!(!any_contains_char(&[], 'a'));
        assert!(any_contains_char(&["test"], 't'));
    }

    #[test]
    fn test_find_long_string() {
        assert_eq!(
            find_long_string(&["hi", "hello", "yo"], 3),
            Some("hello".to_string())
        );
        assert_eq!(find_long_string(&["a", "b"], 5), None);
        assert_eq!(find_long_string(&[], 0), None);
    }
}
