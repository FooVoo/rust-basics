//! Exercise 14: Async Retry Logic - Handling transient failures
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement retry mechanisms
//! - Handle transient failures
//! - Add delays between retries

use tokio::time::{sleep, Duration};

/// Retry an operation up to max_attempts times.
pub async fn retry<F, Fut, T, E>(max_attempts: u32, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
 {
    todo!("Retry an operation up to max_attempts times.")
}

/// Retry with exponential backoff.
pub async fn retry_with_backoff<F, Fut, T, E>(
    max_attempts: u32,
    base_delay_ms: u64,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
 {
    todo!("Implement retry_with_backoff")
}

/// Counter for testing retry logic.
pub struct FailCounter {
    pub count: std::sync::Arc<tokio::sync::Mutex<u32>>,
    pub fail_until: u32,
}

impl FailCounter {
    pub fn new(fail_until: u32) -> Self {
        todo!("Implement new")
    }

    pub async fn try_operation(&self) -> Result<String, String> {
        todo!("Implement try_operation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_success() {
        let counter = FailCounter::new(2);
        let result = retry(5, || counter.try_operation()).await;
        assert_eq!(result, Ok("Success".to_string()));
    }

    #[tokio::test]
    async fn test_retry_failure() {
        let counter = FailCounter::new(10);
        let result = retry(3, || counter.try_operation()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let counter = FailCounter::new(2);
        let result = retry_with_backoff(5, 1, || counter.try_operation()).await;
        assert_eq!(result, Ok("Success".to_string()));
    }
}
