//! Exercise 10: Async Channels - Communication between tasks
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::mpsc channels
//! - Send and receive messages between tasks
//! - Handle channel closure

use tokio::sync::mpsc;

/// Send multiple values through a channel.
pub async fn send_values(values: Vec<i32>) -> Vec<i32> {
    todo!("Implement send_values")
}

/// Process values through a channel with transformation.
pub async fn channel_transform(values: Vec<i32>) -> Vec<i32> {
    todo!("Implement channel_transform")
}

/// Use a channel to communicate between multiple producers and one consumer.
pub async fn multiple_producers(count: usize) -> Vec<i32> {
    todo!("Implement multiple_producers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_values() {
        let result = send_values(vec![1, 2, 3, 4]).await;
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_channel_transform() {
        let result = channel_transform(vec![1, 2, 3]).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_multiple_producers() {
        let result = multiple_producers(5).await;
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }
}
