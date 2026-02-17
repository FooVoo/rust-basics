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
    let mut set = JoinSet::new();
    
    for (value, delay) in tasks {
        set.spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            value * 2
        });
    }
    
    let mut results = vec![];
    while let Some(result) = set.join_next().await {
        results.push(result.unwrap());
    }
    
    results.sort();
    results
}

/// Process results as they arrive.
pub async fn process_as_completed(num_tasks: usize) -> Vec<usize> {
    let mut set = JoinSet::new();
    
    for i in 0..num_tasks {
        let delay = (num_tasks - i) as u64 * 10;
        set.spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            i
        });
    }
    
    let mut completion_order = vec![];
    while let Some(result) = set.join_next().await {
        completion_order.push(result.unwrap());
    }
    
    completion_order
}

/// Spawn tasks conditionally based on results.
pub async fn conditional_spawning(initial_values: Vec<i32>) -> Vec<i32> {
    let mut set = JoinSet::new();
    let mut results = vec![];
    
    for value in initial_values {
        set.spawn(async move {
            sleep(Duration::from_millis(10)).await;
            value * 2
        });
    }
    
    while let Some(result) = set.join_next().await {
        let value = result.unwrap();
        results.push(value);
        
        if value < 50 && results.len() < 10 {
            set.spawn(async move {
                sleep(Duration::from_millis(10)).await;
                value + 10
            });
        }
    }
    
    results.sort();
    results
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
