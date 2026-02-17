//! Exercise 17: Async Barrier - Synchronization point for multiple tasks
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::Barrier
//! - Synchronize multiple tasks
//! - Coordinate concurrent operations

use tokio::sync::Barrier;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

/// Wait for all tasks to reach the barrier before proceeding.
pub async fn synchronized_tasks(num_tasks: usize, delays_ms: Vec<u64>) -> Vec<String> {
    todo!("Implement synchronized_tasks")
}

/// Multi-phase computation with barriers.
pub async fn multi_phase_computation(num_workers: usize) -> Vec<i32> {
    todo!("Implement multi_phase_computation")
}

/// Barrier for batch processing.
pub async fn batch_processing(batches: Vec<Vec<i32>>) -> Vec<i32> {
    todo!("Implement batch_processing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_synchronized_tasks() {
        let results = synchronized_tasks(3, vec![30, 10, 20]).await;
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_multi_phase_computation() {
        let results = multi_phase_computation(4).await;
        assert_eq!(results, vec![5, 15, 25, 35]);
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let batches = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut results = batch_processing(batches).await;
        results.sort();
        assert_eq!(results, vec![6, 15, 24]);
    }
}
