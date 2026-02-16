//! Exercise 01: Option::map - Transform wrapped values
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Option::map combinator
//! - Transform values inside Option
//! - Chain simple transformations

/// Double the value inside an Option.
/// If None, return None.
pub fn double_option(value: Option<i32>) -> Option<i32> {
    value.map(|x| x * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_some() {
        assert_eq!(double_option(Some(5)), Some(10));
        assert_eq!(double_option(Some(0)), Some(0));
        assert_eq!(double_option(Some(-3)), Some(-6));
    }

    #[test]
    fn test_double_none() {
        assert_eq!(double_option(None), None);
    }
}
