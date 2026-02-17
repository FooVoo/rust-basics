//! Exercise 06: Basic Timeout - Adding time limits to operations
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use tokio::time::timeout
//! - Handle timeout errors
//! - Set time limits on async operations

use tokio::time::{sleep, timeout, Duration};

/// Execute an operation with a timeout.
pub async fn with_timeout(delay_ms: u64, timeout_ms: u64) -> Result<String, String>  {
    todo!("Execute an operation with a timeout.")
}

/// Try to parse with a timeout.
pub async fn timed_parse(s: &str, timeout_ms: u64) -> Result<i32, String>  {
    todo!("Try to parse with a timeout.")
}

/// Execute multiple operations with individual timeouts.
pub async fn multiple_timeouts(operations: Vec<u64>, timeout_ms: u64) -> Vec<Result<String, String>>  {
    todo!("Execute multiple operations with individual timeouts.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_timeout_success() {
        let result = with_timeout(10, 100).await;
        assert_eq!(result, Ok("Completed".to_string()));
    }

    #[tokio::test]
    async fn test_with_timeout_failure() {
        let result = with_timeout(100, 10).await;
        assert_eq!(result, Err("Timeout".to_string()));
    }

    #[tokio::test]
    async fn test_timed_parse() {
        assert_eq!(timed_parse("42", 100).await, Ok(42));
        assert!(timed_parse("abc", 100).await.is_err());
    }

    #[tokio::test]
    async fn test_multiple_timeouts() {
        let results = multiple_timeouts(vec![10, 100, 20], 50).await;
        assert_eq!(results[0], Ok("Completed".to_string()));
        assert_eq!(results[1], Err("Timeout".to_string()));
        assert_eq!(results[2], Ok("Completed".to_string()));
    }
}
