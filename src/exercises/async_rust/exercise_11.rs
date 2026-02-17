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
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..num_tasks {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            for _ in 0..increments_per_task {
                let mut count = counter_clone.lock().await;
                *count += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let final_count = counter.lock().await;
    *final_count
}

/// Share a vector between tasks for concurrent modifications.
pub async fn shared_vector(num_tasks: usize) -> Vec<i32> {
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for i in 0..num_tasks {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut vec = data_clone.lock().await;
            vec.push(i as i32);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let mut result = data.lock().await.clone();
    result.sort();
    result
}

/// Implement a simple async task queue.
pub async fn task_queue(tasks: Vec<i32>) -> Vec<i32> {
    let queue = Arc::new(Mutex::new(tasks));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let queue_clone = Arc::clone(&queue);
        let results_clone = Arc::clone(&results);
        
        let handle = tokio::spawn(async move {
            loop {
                let task = {
                    let mut q = queue_clone.lock().await;
                    q.pop()
                };
                
                match task {
                    Some(value) => {
                        let result = value * 2;
                        let mut r = results_clone.lock().await;
                        r.push(result);
                    }
                    None => break,
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let mut final_results = results.lock().await.clone();
    final_results.sort();
    final_results
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
