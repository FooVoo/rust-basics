//! Exercise 05: Multiple Senders
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Clone Sender for multiple threads
//! - Coordinate multiple producers
//! - Aggregate results from multiple sources

use std::sync::mpsc;
use std::thread;

/// Spawn n_threads threads, each sending its thread index to a channel.
/// Return the sum of all received values.
pub fn sum_from_threads(n_threads: usize) -> usize {
    let (tx, rx) = mpsc::channel();

    for i in 0..n_threads {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
        });
    }

    drop(tx); // Drop original sender so rx.iter() can terminate

    rx.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_from_threads() {
        assert_eq!(sum_from_threads(0), 0);
        assert_eq!(sum_from_threads(1), 0);
        assert_eq!(sum_from_threads(5), 0 + 1 + 2 + 3 + 4);
    }

    #[test]
    fn test_sum_from_threads_larger() {
        let result = sum_from_threads(10);
        assert_eq!(result, 45); // 0+1+2+...+9
    }
}
