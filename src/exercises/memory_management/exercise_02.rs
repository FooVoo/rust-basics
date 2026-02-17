//! Exercise 02: Immutable Borrowing - Share data without taking ownership
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use immutable references to borrow data
//! - Allow multiple immutable borrows simultaneously
//! - Avoid ownership transfer when only reading data

/// Calculate the length of a string without taking ownership.
pub fn calculate_length(s: &String) -> usize  {
    todo!("Calculate the length of a string without taking ownership.")
}

/// Check if a string starts with a given prefix.
pub fn starts_with(s: &String, prefix: &str) -> bool  {
    todo!("Check if a string starts with a given prefix.")
}

/// Get the first character of a string if it exists.
pub fn first_char(s: &String) -> Option<char>  {
    todo!("Get the first character of a string if it exists.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq!(len, 5);
        // s is still valid - we only borrowed it
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_starts_with() {
        let s = String::from("hello world");
        assert!(starts_with(&s, "hello"));
        assert!(!starts_with(&s, "world"));
        // s is still valid
        assert_eq!(s.len(), 11);
    }

    #[test]
    fn test_first_char() {
        let s = String::from("abc");
        assert_eq!(first_char(&s), Some('a'));
        
        let empty = String::new();
        assert_eq!(first_char(&empty), None);
    }

    #[test]
    fn test_multiple_borrows() {
        let s = String::from("test");
        let len1 = calculate_length(&s);
        let len2 = calculate_length(&s);
        let first = first_char(&s);
        
        assert_eq!(len1, len2);
        assert_eq!(first, Some('t'));
    }
}
