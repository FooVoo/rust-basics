//! Exercise 18: Async Notify - Wake up waiting tasks
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::Notify
//! - Coordinate between tasks with notifications
//! - Implement async waiting patterns

use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

/// Wait for a notification before proceeding.
pub async fn wait_for_signal(delay_ms: u64) -> String  {
    todo!("Wait for a notification before proceeding.")
}

/// Producer-consumer pattern with Notify.
pub async fn producer_consumer(items: Vec<i32>) -> Vec<i32>  {
    todo!("Producer-consumer pattern with Notify.")
}

/// Broadcast notification to multiple waiters.
pub async fn broadcast_notification(num_waiters: usize) -> Vec<String>  {
    todo!("Broadcast notification to multiple waiters.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_for_signal() {
        let result = wait_for_signal(10).await;
        assert_eq!(result, "Signal received");
    }

    #[tokio::test]
    async fn test_producer_consumer() {
        let results = producer_consumer(vec![1, 2, 3]).await;
        assert!(results.contains(&2));
        assert!(results.contains(&4));
        assert!(results.contains(&6));
    }

    #[tokio::test]
    async fn test_broadcast_notification() {
        let results = broadcast_notification(3).await;
        assert_eq!(results.len(), 3);
    }
}
