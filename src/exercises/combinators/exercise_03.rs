//! Exercise 03: Option::or_else - Provide fallback values
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Option::or_else combinator
//! - Provide alternative computations
//! - Handle fallback scenarios

/// Get a value from primary source or fallback to secondary.
pub fn get_value_with_fallback(primary: Option<i32>, fallback: Option<i32>) -> Option<i32> {
    primary.or_else(|| fallback)
}

/// Get first available value from multiple sources.
pub fn first_available(sources: Vec<Option<i32>>) -> Option<i32> {
    sources.into_iter().find(|opt| opt.is_some()).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_primary_exists() {
        assert_eq!(get_value_with_fallback(Some(10), Some(20)), Some(10));
    }

    #[test]
    fn test_fallback_use_secondary() {
        assert_eq!(get_value_with_fallback(None, Some(20)), Some(20));
    }

    #[test]
    fn test_fallback_both_none() {
        assert_eq!(get_value_with_fallback(None, None), None);
    }

    #[test]
    fn test_first_available() {
        assert_eq!(first_available(vec![None, Some(5), Some(10)]), Some(5));
        assert_eq!(first_available(vec![None, None, None]), None);
        assert_eq!(first_available(vec![Some(1)]), Some(1));
    }
}
