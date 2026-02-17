//! Exercise 16: Async Semaphore - Limiting concurrent operations
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::Semaphore
//! - Limit concurrent task execution
//! - Manage resource access

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

/// Execute tasks with a concurrency limit.
pub async fn limited_concurrency(tasks: Vec<i32>, max_concurrent: usize) -> Vec<i32> {
    todo!("Implement limited_concurrency")
}

/// Rate-limited API calls simulation.
pub async fn rate_limited_calls(num_calls: usize, rate_limit: usize) -> Vec<usize> {
    todo!("Implement rate_limited_calls")
}

/// Shared resource pool with semaphore.
pub async fn resource_pool(requests: usize, pool_size: usize) -> Vec<String> {
    todo!("Implement resource_pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_limited_concurrency() {
        let result = limited_concurrency(vec![1, 2, 3, 4, 5], 2).await;
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_rate_limited_calls() {
        let results = rate_limited_calls(5, 2).await;
        assert_eq!(results.len(), 5);
        assert!(results.contains(&0));
        assert!(results.contains(&4));
    }

    #[tokio::test]
    async fn test_resource_pool() {
        let results = resource_pool(3, 2).await;
        assert_eq!(results.len(), 3);
        assert!(results[0].contains("Request"));
    }
}
