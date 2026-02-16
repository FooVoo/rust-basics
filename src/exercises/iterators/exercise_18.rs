//! Exercise 18: Try Collect - Error handling with iterators
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use collect() with Result types
//! - Handle errors in iterator pipelines
//! - Short-circuit on errors

/// Parse all strings, returning error on first failure.
pub fn parse_all(strings: &[&str]) -> Result<Vec<i32>, String> {
    strings
        .iter()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}

/// Divide all pairs, failing on division by zero.
pub fn safe_divide_all(pairs: &[(i32, i32)]) -> Result<Vec<i32>, String> {
    pairs
        .iter()
        .map(|&(a, b)| {
            if b == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        })
        .collect()
}

/// Convert strings to lengths, failing if any string is empty.
pub fn lengths_no_empty(strings: &[&str]) -> Result<Vec<usize>, String> {
    strings
        .iter()
        .map(|s| {
            if s.is_empty() {
                Err("Empty string found".to_string())
            } else {
                Ok(s.len())
            }
        })
        .collect()
}

/// Parse and validate that all numbers are positive.
pub fn parse_positive(strings: &[&str]) -> Result<Vec<i32>, String> {
    strings
        .iter()
        .map(|s| {
            let num = s.parse::<i32>().map_err(|e| e.to_string())?;
            if num <= 0 {
                Err(format!("{} is not positive", num))
            } else {
                Ok(num)
            }
        })
        .collect()
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
