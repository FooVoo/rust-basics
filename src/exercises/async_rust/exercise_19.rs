//! Exercise 19: Async Watch Channel - Broadcasting state changes
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::watch channel
//! - Broadcast state updates to multiple receivers
//! - React to state changes

use tokio::sync::watch;
use tokio::time::{sleep, Duration};

/// Broadcast value updates to multiple subscribers.
pub async fn broadcast_updates(updates: Vec<i32>) -> Vec<Vec<i32>> {
    let (tx, mut rx1) = watch::channel(0);
    let mut rx2 = tx.subscribe();
    
    let handle1 = tokio::spawn(async move {
        let mut values = vec![*rx1.borrow()];
        while rx1.changed().await.is_ok() {
            values.push(*rx1.borrow());
        }
        values
    });
    
    let handle2 = tokio::spawn(async move {
        let mut values = vec![*rx2.borrow()];
        while rx2.changed().await.is_ok() {
            values.push(*rx2.borrow());
        }
        values
    });
    
    for value in updates {
        sleep(Duration::from_millis(10)).await;
        let _ = tx.send(value);
    }
    
    drop(tx);
    
    vec![handle1.await.unwrap(), handle2.await.unwrap()]
}

/// Monitor state changes and react to them.
pub async fn state_monitor(states: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = watch::channel("initial".to_string());
    
    let monitor = tokio::spawn(async move {
        let mut observed = vec![];
        observed.push(rx.borrow().clone());
        
        while rx.changed().await.is_ok() {
            observed.push(rx.borrow().clone());
        }
        observed
    });
    
    for state in states {
        sleep(Duration::from_millis(10)).await;
        let _ = tx.send(state);
    }
    
    drop(tx);
    monitor.await.unwrap()
}

/// Track latest value with watch channel.
pub async fn latest_value_tracker(values: Vec<i32>, delay_ms: u64) -> i32 {
    let (tx, rx) = watch::channel(0);
    
    tokio::spawn(async move {
        for value in values {
            sleep(Duration::from_millis(10)).await;
            let _ = tx.send(value);
        }
    });
    
    sleep(Duration::from_millis(delay_ms)).await;
    *rx.borrow()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_broadcast_updates() {
        let results = broadcast_updates(vec![1, 2, 3]).await;
        assert_eq!(results.len(), 2);
        assert!(results[0].contains(&3));
        assert!(results[1].contains(&3));
    }

    #[tokio::test]
    async fn test_state_monitor() {
        let results = state_monitor(vec!["state1".to_string(), "state2".to_string()]).await;
        assert!(results.contains(&"initial".to_string()));
        assert!(results.contains(&"state2".to_string()));
    }

    #[tokio::test]
    async fn test_latest_value_tracker() {
        let result = latest_value_tracker(vec![1, 2, 3, 4, 5], 100).await;
        assert_eq!(result, 5);
    }
}
