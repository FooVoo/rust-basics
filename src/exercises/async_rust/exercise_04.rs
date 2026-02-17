//! Exercise 04: Basic Task Spawning - Creating concurrent tasks
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use tokio::spawn to create tasks
//! - Await on JoinHandle
//! - Understand basic concurrency

use tokio::task;

/// Spawn a task that computes a value.
pub async fn spawn_computation(value: i32) -> i32 {
    todo!("Implement spawn_computation")
}

/// Spawn two tasks and return their sum.
pub async fn spawn_and_sum(a: i32, b: i32) -> i32 {
    todo!("Implement spawn_and_sum")
}

/// Spawn multiple tasks and collect results.
pub async fn spawn_multiple(values: Vec<i32>) -> Vec<i32> {
    todo!("Implement spawn_multiple")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_computation() {
        assert_eq!(spawn_computation(5).await, 10);
        assert_eq!(spawn_computation(0).await, 0);
    }

    #[tokio::test]
    async fn test_spawn_and_sum() {
        assert_eq!(spawn_and_sum(2, 3).await, 13);
        assert_eq!(spawn_and_sum(5, 5).await, 25);
    }

    #[tokio::test]
    async fn test_spawn_multiple() {
        let result = spawn_multiple(vec![1, 2, 3]).await;
        assert_eq!(result, vec![11, 12, 13]);
    }
}
