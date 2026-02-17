//! Exercise 25: JoinSet - Managing dynamic task sets
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use tokio::task::JoinSet
//! - Manage dynamic collections of tasks
//! - Process results as they complete

use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

/// Spawn tasks dynamically and collect results.
pub async fn dynamic_task_set(tasks: Vec<(i32, u64)>) -> Vec<i32> {
    todo!("Implement dynamic_task_set")
}

/// Process results as they arrive.
pub async fn process_as_completed(num_tasks: usize) -> Vec<usize> {
    todo!("Implement process_as_completed")
}

/// Spawn tasks conditionally based on results.
pub async fn conditional_spawning(initial_values: Vec<i32>) -> Vec<i32> {
    todo!("Implement conditional_spawning")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_task_set() {
        let tasks = vec![(1, 20), (2, 10), (3, 30)];
        let result = dynamic_task_set(tasks).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_process_as_completed() {
        let result = process_as_completed(3).await;
        assert_eq!(result.len(), 3);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[tokio::test]
    async fn test_conditional_spawning() {
        let result = conditional_spawning(vec![5, 10]).await;
        assert!(result.contains(&10));
        assert!(result.contains(&20));
    }
}
