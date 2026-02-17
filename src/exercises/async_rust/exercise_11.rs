//! Exercise 11: Async Mutex - Shared state between tasks
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::Mutex for shared state
//! - Understand async locking
//! - Coordinate between multiple tasks

use tokio::sync::Mutex;
use std::sync::Arc;

/// Increment a shared counter from multiple tasks.
pub async fn concurrent_counter(num_tasks: usize, increments_per_task: usize) -> i32 {
    todo!("Implement concurrent_counter")
}

/// Share a vector between tasks for concurrent modifications.
pub async fn shared_vector(num_tasks: usize) -> Vec<i32> {
    todo!("Implement shared_vector")
}

/// Implement a simple async task queue.
pub async fn task_queue(tasks: Vec<i32>) -> Vec<i32> {
    todo!("Implement task_queue")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_counter() {
        let result = concurrent_counter(10, 10).await;
        assert_eq!(result, 100);
    }

    #[tokio::test]
    async fn test_shared_vector() {
        let result = shared_vector(5).await;
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_task_queue() {
        let result = task_queue(vec![1, 2, 3, 4, 5]).await;
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}
