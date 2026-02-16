//! Exercise 04: Basic Channel Communication
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand channels (mpsc)
//! - Send and receive messages
//! - Work with Sender and Receiver

use std::sync::mpsc;
use std::thread;

/// Spawn a thread that sends numbers 0..n through a channel.
/// Collect and return all received numbers.
pub fn channel_numbers(n: i32) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..n {
            tx.send(i).unwrap();
        }
    });

    rx.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_numbers() {
        assert_eq!(channel_numbers(0), vec![]);
        assert_eq!(channel_numbers(1), vec![0]);
        assert_eq!(channel_numbers(5), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_channel_numbers_larger() {
        let result = channel_numbers(10);
        assert_eq!(result.len(), 10);
        assert_eq!(result[9], 9);
    }
}
