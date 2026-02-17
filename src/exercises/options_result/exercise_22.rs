//! Exercise 22: Option and Result flatten
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use flatten() to collapse nested Options
//! - Understand when flatten is useful
//! - Avoid deeply nested structures

/// Flatten a nested Option.
pub fn flatten_option(opt: Option<Option<i32>>) -> Option<i32>  {
    todo!("Flatten a nested Option.")
}

/// Parse a string that might contain another parseable value.
pub fn parse_nested(s: &str) -> Option<i32>  {
    todo!("Parse a string that might contain another parseable value.")
}

/// Get a value from a nested structure.
pub fn get_nested_value(
    outer: Option<Vec<i32>>,
    index: usize,
) -> Option<i32>  {
    todo!("Get a value from a nested structure.")
}

/// Flatten a vector of Options, keeping only Some values.
pub fn flatten_vec_options(vec: Vec<Option<i32>>) -> Vec<i32>  {
    todo!("Flatten a vector of Options, keeping only Some values.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_option() {
        assert_eq!(flatten_option(Some(Some(42))), Some(42));
        assert_eq!(flatten_option(Some(None)), None);
        assert_eq!(flatten_option(None), None);
    }

    #[test]
    fn test_parse_nested() {
        assert_eq!(parse_nested("42"), Some(42));
        assert_eq!(parse_nested("abc"), None);
    }

    #[test]
    fn test_get_nested_value() {
        assert_eq!(get_nested_value(Some(vec![1, 2, 3]), 1), Some(2));
        assert_eq!(get_nested_value(Some(vec![1, 2, 3]), 5), None);
        assert_eq!(get_nested_value(None, 0), None);
    }

    #[test]
    fn test_flatten_vec_options() {
        assert_eq!(
            flatten_vec_options(vec![Some(1), None, Some(2), Some(3)]),
            vec![1, 2, 3]
        );
        assert_eq!(flatten_vec_options(vec![None, None]), vec![]);
    }
}
