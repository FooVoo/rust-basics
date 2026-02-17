//! Exercise 18: Try Collect - Error handling with iterators
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use collect() with Result types
//! - Handle errors in iterator pipelines
//! - Short-circuit on errors

/// Parse all strings, returning error on first failure.
pub fn parse_all(strings: &[&str]) -> Result<Vec<i32>, String>  {
    todo!("Parse all strings, returning error on first failure.")
}

/// Divide all pairs, failing on division by zero.
pub fn safe_divide_all(pairs: &[(i32, i32)]) -> Result<Vec<i32>, String>  {
    todo!("Divide all pairs, failing on division by zero.")
}

/// Convert strings to lengths, failing if any string is empty.
pub fn lengths_no_empty(strings: &[&str]) -> Result<Vec<usize>, String>  {
    todo!("Convert strings to lengths, failing if any string is empty.")
}

/// Parse and validate that all numbers are positive.
pub fn parse_positive(strings: &[&str]) -> Result<Vec<i32>, String>  {
    todo!("Parse and validate that all numbers are positive.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_success() {
        assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert_eq!(parse_all(&[]), Ok(vec![]));
    }

    #[test]
    fn test_parse_all_failure() {
        assert!(parse_all(&["1", "abc", "3"]).is_err());
        assert!(parse_all(&["abc"]).is_err());
    }

    #[test]
    fn test_safe_divide_all_success() {
        assert_eq!(safe_divide_all(&[(10, 2), (15, 3)]), Ok(vec![5, 5]));
        assert_eq!(safe_divide_all(&[]), Ok(vec![]));
    }

    #[test]
    fn test_safe_divide_all_failure() {
        assert!(safe_divide_all(&[(10, 2), (15, 0)]).is_err());
        assert!(safe_divide_all(&[(10, 0)]).is_err());
    }

    #[test]
    fn test_lengths_no_empty_success() {
        assert_eq!(lengths_no_empty(&["a", "bb", "ccc"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_lengths_no_empty_failure() {
        assert!(lengths_no_empty(&["a", "", "ccc"]).is_err());
        assert!(lengths_no_empty(&[""]).is_err());
    }

    #[test]
    fn test_parse_positive_success() {
        assert_eq!(parse_positive(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_positive_failure() {
        assert!(parse_positive(&["1", "0", "3"]).is_err());
        assert!(parse_positive(&["1", "-5", "3"]).is_err());
        assert!(parse_positive(&["abc"]).is_err());
    }
}
