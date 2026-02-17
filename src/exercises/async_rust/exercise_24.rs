//! Exercise 24: Cancellation Tokens - Managing task cancellation
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use tokio_util::sync::CancellationToken
//! - Implement graceful cancellation
//! - Coordinate task shutdown

use tokio_util::sync::CancellationToken;
use tokio::time::{sleep, Duration};

/// Run a task until cancelled.
pub async fn cancellable_task(token: CancellationToken) -> i32 {
    let mut count = 0;
    
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                break;
            }
            _ = sleep(Duration::from_millis(10)) => {
                count += 1;
            }
        }
    }
    
    count
}

/// Multiple tasks with shared cancellation.
pub async fn coordinated_cancellation(num_tasks: usize, run_duration_ms: u64) -> Vec<i32> {
    let token = CancellationToken::new();
    let mut handles = vec![];
    
    for _ in 0..num_tasks {
        let task_token = token.clone();
        let handle = tokio::spawn(async move {
            cancellable_task(task_token).await
        });
        handles.push(handle);
    }
    
    sleep(Duration::from_millis(run_duration_ms)).await;
    token.cancel();
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    results
}

/// Hierarchical cancellation with child tokens.
pub async fn hierarchical_cancellation() -> (bool, bool) {
    let parent_token = CancellationToken::new();
    let child_token = parent_token.child_token();
    
    let parent_handle = tokio::spawn({
        let token = parent_token.clone();
        async move {
            token.cancelled().await;
            true
        }
    });
    
    let child_handle = tokio::spawn(async move {
        child_token.cancelled().await;
        true
    });
    
    sleep(Duration::from_millis(10)).await;
    parent_token.cancel();
    
    let parent_cancelled = parent_handle.await.unwrap();
    let child_cancelled = child_handle.await.unwrap();
    
    (parent_cancelled, child_cancelled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cancellable_task() {
        let token = CancellationToken::new();
        let task_token = token.clone();
        
        let handle = tokio::spawn(async move {
            cancellable_task(task_token).await
        });
        
        sleep(Duration::from_millis(50)).await;
        token.cancel();
        
        let count = handle.await.unwrap();
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_coordinated_cancellation() {
        let results = coordinated_cancellation(3, 50).await;
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|&c| c > 0));
    }

    #[tokio::test]
    async fn test_hierarchical_cancellation() {
        let (parent, child) = hierarchical_cancellation().await;
        assert!(parent);
        assert!(child);
    }
}
