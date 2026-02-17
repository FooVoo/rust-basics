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
    let barrier = Arc::new(Barrier::new(num_tasks));
    let mut handles = vec![];
    
    for (i, delay) in delays_ms.into_iter().enumerate() {
        let barrier_clone = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            barrier_clone.wait().await;
            format!("Task {} completed", i)
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    results
}

/// Multi-phase computation with barriers.
pub async fn multi_phase_computation(num_workers: usize) -> Vec<i32> {
    let barrier1 = Arc::new(Barrier::new(num_workers));
    let barrier2 = Arc::new(Barrier::new(num_workers));
    let results = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for i in 0..num_workers {
        let b1 = Arc::clone(&barrier1);
        let b2 = Arc::clone(&barrier2);
        let res = Arc::clone(&results);
        
        let handle = tokio::spawn(async move {
            let phase1 = i as i32 * 10;
            b1.wait().await;
            
            let phase2 = phase1 + 5;
            b2.wait().await;
            
            let mut r = res.lock().await;
            r.push(phase2);
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

/// Barrier for batch processing.
pub async fn batch_processing(batches: Vec<Vec<i32>>) -> Vec<i32> {
    let num_batches = batches.len();
    let barrier = Arc::new(Barrier::new(num_batches));
    let results = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for batch in batches {
        let barrier_clone = Arc::clone(&barrier);
        let results_clone = Arc::clone(&results);
        
        let handle = tokio::spawn(async move {
            let sum: i32 = batch.iter().sum();
            barrier_clone.wait().await;
            
            let mut r = results_clone.lock().await;
            r.push(sum);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    results.lock().await.clone()
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
