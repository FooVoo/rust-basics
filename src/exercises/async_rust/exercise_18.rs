//! Exercise 18: Async Notify - Wake up waiting tasks
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::Notify
//! - Coordinate between tasks with notifications
//! - Implement async waiting patterns

use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

/// Wait for a notification before proceeding.
pub async fn wait_for_signal(delay_ms: u64) -> String {
    let notify = Arc::new(Notify::new());
    let notify_clone = Arc::clone(&notify);
    
    tokio::spawn(async move {
        sleep(Duration::from_millis(delay_ms)).await;
        notify_clone.notify_one();
    });
    
    notify.notified().await;
    "Signal received".to_string()
}

/// Producer-consumer pattern with Notify.
pub async fn producer_consumer(items: Vec<i32>) -> Vec<i32> {
    let notify = Arc::new(Notify::new());
    let queue = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let results = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    
    let producer_notify = Arc::clone(&notify);
    let producer_queue = Arc::clone(&queue);
    let producer = tokio::spawn(async move {
        for item in items {
            let mut q = producer_queue.lock().await;
            q.push(item);
            drop(q);
            producer_notify.notify_one();
            sleep(Duration::from_millis(10)).await;
        }
    });
    
    let consumer_notify = Arc::clone(&notify);
    let consumer_queue = Arc::clone(&queue);
    let consumer_results = Arc::clone(&results);
    let consumer = tokio::spawn(async move {
        loop {
            consumer_notify.notified().await;
            
            let item = {
                let mut q = consumer_queue.lock().await;
                q.pop()
            };
            
            match item {
                Some(value) => {
                    let mut r = consumer_results.lock().await;
                    r.push(value * 2);
                }
                None => break,
            }
        }
    });
    
    producer.await.unwrap();
    sleep(Duration::from_millis(50)).await;
    notify.notify_one();
    
    drop(consumer);
    
    results.lock().await.clone()
}

/// Broadcast notification to multiple waiters.
pub async fn broadcast_notification(num_waiters: usize) -> Vec<String> {
    let notify = Arc::new(Notify::new());
    let mut handles = vec![];
    
    for i in 0..num_waiters {
        let notify_clone = Arc::clone(&notify);
        let handle = tokio::spawn(async move {
            notify_clone.notified().await;
            format!("Waiter {} notified", i)
        });
        handles.push(handle);
    }
    
    sleep(Duration::from_millis(10)).await;
    
    for _ in 0..num_waiters {
        notify.notify_one();
    }
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_for_signal() {
        let result = wait_for_signal(10).await;
        assert_eq!(result, "Signal received");
    }

    #[tokio::test]
    async fn test_producer_consumer() {
        let results = producer_consumer(vec![1, 2, 3]).await;
        assert!(results.contains(&2));
        assert!(results.contains(&4));
        assert!(results.contains(&6));
    }

    #[tokio::test]
    async fn test_broadcast_notification() {
        let results = broadcast_notification(3).await;
        assert_eq!(results.len(), 3);
    }
}
