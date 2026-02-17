//! Exercise 18: Bounded Channel
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use sync_channel for bounded queues
//! - Understand backpressure
//! - Handle blocking sends

use std::sync::mpsc;
use std::thread;

/// Create a bounded channel and test that it blocks when full.
/// Producer sends n_items, consumer receives with delay.
/// Return all received items.
pub fn bounded_channel_test(capacity: usize, n_items: usize) -> Vec<i32> {
    todo!("Implement bounded_channel_test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_channel() {
        let result = bounded_channel_test(5, 10);
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_bounded_channel_small_capacity() {
        let result = bounded_channel_test(1, 5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_bounded_channel_large_capacity() {
        let result = bounded_channel_test(100, 10);
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
