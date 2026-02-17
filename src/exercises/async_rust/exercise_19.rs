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
    todo!("Implement broadcast_updates")
}

/// Monitor state changes and react to them.
pub async fn state_monitor(states: Vec<String>) -> Vec<String> {
    todo!("Implement state_monitor")
}

/// Track latest value with watch channel.
pub async fn latest_value_tracker(values: Vec<i32>, delay_ms: u64) -> i32 {
    todo!("Implement latest_value_tracker")
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
