//! Exercise 20: Async Oneshot Channel - Single-value communication
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::oneshot channel
//! - Send single values between tasks
//! - Handle channel closure and cancellation

use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

/// Send a computed value through oneshot channel.
pub async fn compute_and_send(value: i32, delay_ms: u64) -> i32 {
    todo!("Implement compute_and_send")
}

/// Request-response pattern with oneshot.
pub async fn request_response(requests: Vec<i32>) -> Vec<i32> {
    todo!("Implement request_response")
}

/// Handle oneshot cancellation.
pub async fn with_cancellation(value: i32, should_cancel: bool) -> Result<i32, String> {
    todo!("Implement with_cancellation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compute_and_send() {
        let result = compute_and_send(10, 10).await;
        assert_eq!(result, 20);
    }

    #[tokio::test]
    async fn test_request_response() {
        let results = request_response(vec![1, 2, 3]).await;
        assert_eq!(results, vec![101, 102, 103]);
    }

    #[tokio::test]
    async fn test_with_cancellation() {
        assert_eq!(with_cancellation(5, false).await, Ok(10));
        assert_eq!(with_cancellation(5, true).await, Err("Cancelled".to_string()));
    }
}
