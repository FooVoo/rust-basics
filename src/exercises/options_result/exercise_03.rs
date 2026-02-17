//! Exercise 03: Option is_some and is_none
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use is_some() to check if Option contains a value
//! - Use is_none() to check if Option is None
//! - Avoid unwrapping when only checking presence

/// Check if a string contains a given character.
/// Returns true if the character is found, false otherwise.
pub fn contains_char(s: &str, c: char) -> bool {
    todo!("Implement contains_char")
}

/// Check if a vector is empty by checking if pop returns None.
pub fn is_empty_via_pop(mut v: Vec<i32>) -> bool {
    todo!("Implement is_empty_via_pop")
}

/// Count how many Options in a slice contain a value.
pub fn count_some(options: &[Option<i32>]) -> usize {
    todo!("Implement count_some")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_char() {
        assert!(contains_char("hello", 'e'));
        assert!(contains_char("rust", 'r'));
        assert!(!contains_char("hello", 'x'));
        assert!(!contains_char("", 'a'));
    }

    #[test]
    fn test_is_empty_via_pop() {
        assert!(is_empty_via_pop(vec![]));
        assert!(!is_empty_via_pop(vec![1]));
        assert!(!is_empty_via_pop(vec![1, 2, 3]));
    }

    #[test]
    fn test_count_some() {
        assert_eq!(count_some(&[Some(1), None, Some(2), None, Some(3)]), 3);
        assert_eq!(count_some(&[None, None, None]), 0);
        assert_eq!(count_some(&[Some(1), Some(2), Some(3)]), 3);
        assert_eq!(count_some(&[]), 0);
    }
}
