//! Exercise 05: Joining Futures - Concurrent execution with join
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use tokio::join! macro
//! - Execute futures concurrently
//! - Understand the difference between sequential and concurrent execution

use tokio::time::{sleep, Duration};

/// Execute two async functions concurrently and return both results.
pub async fn concurrent_multiply(a: i32, b: i32) -> (i32, i32) {
    todo!("Implement concurrent_multiply")
}

/// Execute three delayed operations concurrently.
pub async fn concurrent_delays(delay_ms: u64) -> (String, String, String) {
    todo!("Implement concurrent_delays")
}

/// Compute sum of values concurrently.
pub async fn concurrent_sum(values: Vec<i32>) -> i32 {
    todo!("Implement concurrent_sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_multiply() {
        assert_eq!(concurrent_multiply(5, 10).await, (10, 30));
    }

    #[tokio::test]
    async fn test_concurrent_delays() {
        let (first, second, third) = concurrent_delays(10).await;
        assert_eq!(first, "First");
        assert_eq!(second, "Second");
        assert_eq!(third, "Third");
    }

    #[tokio::test]
    async fn test_concurrent_sum() {
        assert_eq!(concurrent_sum(vec![1, 2, 3, 4, 5, 6]).await, 21);
    }
}
