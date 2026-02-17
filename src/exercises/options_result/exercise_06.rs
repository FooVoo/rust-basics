//! Exercise 06: Option map
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use map() to transform Option values
//! - Chain operations on Option
//! - Avoid explicit pattern matching

/// Double a number if it exists in the Option.
pub fn double_if_exists(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

/// Convert an optional string to uppercase.
pub fn uppercase_if_exists(opt: Option<String>) -> Option<String> {
    opt.map(|s| s.to_uppercase())
}

/// Get the length of an optional string.
pub fn length_if_exists(opt: Option<&str>) -> Option<usize> {
    opt.map(|s| s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_if_exists() {
        assert_eq!(double_if_exists(Some(5)), Some(10));
        assert_eq!(double_if_exists(Some(0)), Some(0));
        assert_eq!(double_if_exists(None), None);
    }

    #[test]
    fn test_uppercase_if_exists() {
        assert_eq!(
            uppercase_if_exists(Some(String::from("hello"))),
            Some(String::from("HELLO"))
        );
        assert_eq!(uppercase_if_exists(None), None);
    }

    #[test]
    fn test_length_if_exists() {
        assert_eq!(length_if_exists(Some("hello")), Some(5));
        assert_eq!(length_if_exists(Some("")), Some(0));
        assert_eq!(length_if_exists(None), None);
    }
}
