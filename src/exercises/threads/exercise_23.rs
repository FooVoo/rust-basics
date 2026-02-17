//! Exercise 23: Basic Atomic Operations
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use AtomicUsize for lock-free counters
//! - Understand atomic operations
//! - Work with Ordering semantics

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Increment an atomic counter from multiple threads.
pub fn atomic_counter(n_threads: usize, increments_per_thread: usize) -> usize  {
    todo!("Increment an atomic counter from multiple threads.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter() {
        assert_eq!(atomic_counter(4, 250), 1000);
        assert_eq!(atomic_counter(10, 100), 1000);
    }

    #[test]
    fn test_atomic_counter_single_thread() {
        assert_eq!(atomic_counter(1, 500), 500);
    }

    #[test]
    fn test_atomic_counter_many_threads() {
        assert_eq!(atomic_counter(100, 10), 1000);
    }
}
