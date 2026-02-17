//! Exercise 12: Try Join - Concurrent operations with error handling
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::try_join! for fallible operations
//! - Handle errors in concurrent operations
//! - Stop on first error

use tokio::time::{sleep, Duration};

/// Execute two fallible operations concurrently.
pub async fn try_parse_both(s1: &str, s2: &str) -> Result<(i32, i32), String> {
    todo!("Implement try_parse_both")
}

/// Concurrent division operations with error handling.
pub async fn concurrent_divisions(operations: Vec<(i32, i32)>) -> Result<Vec<i32>, String> {
    todo!("Implement concurrent_divisions")
}

async fn divide(a: i32, b: i32) -> Result<i32, String> {
    todo!("Implement divide")
}

/// Try to parse multiple strings concurrently, fail fast on error.
pub async fn parse_all_or_none(strings: Vec<String>) -> Result<Vec<i32>, String> {
    todo!("Implement parse_all_or_none")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_try_parse_both() {
        assert_eq!(try_parse_both("10", "20").await, Ok((10, 20)));
        assert!(try_parse_both("10", "abc").await.is_err());
        assert!(try_parse_both("abc", "20").await.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_divisions() {
        let result = concurrent_divisions(vec![(10, 2), (20, 4), (30, 5)]).await;
        assert_eq!(result, Ok(vec![5, 5, 6]));
        
        let result = concurrent_divisions(vec![(10, 2), (20, 0), (30, 5)]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_parse_all_or_none() {
        let result = parse_all_or_none(vec!["1".to_string(), "2".to_string(), "3".to_string()]).await;
        assert_eq!(result, Ok(vec![1, 2, 3]));
        
        let result = parse_all_or_none(vec!["1".to_string(), "abc".to_string(), "3".to_string()]).await;
        assert!(result.is_err());
    }
}
