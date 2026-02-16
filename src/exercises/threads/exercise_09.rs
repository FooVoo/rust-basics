//! Exercise 09: Basic Mutex
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand Mutex for shared mutable state
//! - Lock and unlock operations
//! - Work with MutexGuard

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter from multiple threads using Mutex.
/// Spawn n_threads threads, each incrementing the counter n_times.
pub fn concurrent_counter(n_threads: usize, n_times: usize) -> usize {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..n_threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..n_times {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_counter() {
        assert_eq!(concurrent_counter(1, 100), 100);
        assert_eq!(concurrent_counter(4, 250), 1000);
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_concurrent_counter_edge_cases() {
        assert_eq!(concurrent_counter(0, 100), 0);
        assert_eq!(concurrent_counter(5, 0), 0);
    }
}
