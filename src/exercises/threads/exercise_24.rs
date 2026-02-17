//! Exercise 24: Compare-and-Swap
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use compare_exchange operations
//! - Implement lock-free algorithms
//! - Handle CAS failures and retries

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Increment a counter only if current value is less than max.
/// Use compare-and-swap to handle races.
pub fn atomic_bounded_increment(n_threads: usize, max_value: usize) -> usize {
    todo!("Implement atomic_bounded_increment")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_bounded_increment() {
        assert_eq!(atomic_bounded_increment(10, 100), 100);
        assert_eq!(atomic_bounded_increment(5, 50), 50);
    }

    #[test]
    fn test_atomic_bounded_increment_exact_max() {
        assert_eq!(atomic_bounded_increment(20, 10), 10);
    }

    #[test]
    fn test_atomic_bounded_increment_zero_max() {
        assert_eq!(atomic_bounded_increment(5, 0), 0);
    }
}
