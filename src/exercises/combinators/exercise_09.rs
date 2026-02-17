//! Exercise 09: Chaining Option combinators
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Chain multiple Option combinators
//! - Build transformation pipelines
//! - Handle complex Option flows

/// Parse, validate, and transform in one chain.
/// Parse string to i32, keep only positive, then double.
pub fn parse_positive_and_double(s: &str) -> Option<i32>  {
    todo!("Parse string to i32, keep only positive, then double.")
}

/// Extract, validate, and compute.
/// Get first element, ensure it's even, then square it.
pub fn first_even_squared(numbers: &[i32]) -> Option<i32>  {
    todo!("Get first element, ensure it's even, then square it.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_and_double() {
        assert_eq!(parse_positive_and_double("5"), Some(10));
        assert_eq!(parse_positive_and_double("10"), Some(20));
        assert_eq!(parse_positive_and_double("-5"), None);
        assert_eq!(parse_positive_and_double("0"), None);
        assert_eq!(parse_positive_and_double("abc"), None);
    }

    #[test]
    fn test_first_even_squared() {
        assert_eq!(first_even_squared(&[4, 3, 2]), Some(16));
        assert_eq!(first_even_squared(&[2, 4, 6]), Some(4));
        assert_eq!(first_even_squared(&[1, 3, 5]), None);
        assert_eq!(first_even_squared(&[]), None);
    }
}
