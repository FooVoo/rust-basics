//! Exercise 27: Relaxed Memory Ordering
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand different Ordering modes
//! - Use Relaxed ordering appropriately
//! - Balance performance and correctness

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Multiple threads increment counters with different orderings.
/// Demonstrate Relaxed ordering for independent operations.
pub fn relaxed_counters(n_threads: usize) -> (usize, usize) {
    todo!("Implement relaxed_counters")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relaxed_counters() {
        let (c1, c2) = relaxed_counters(10);
        assert_eq!(c1, 100); // 10 threads * 10 increments
        assert_eq!(c2, 25);  // 5 threads * 5 increments
    }

    #[test]
    fn test_relaxed_counters_small() {
        let (c1, c2) = relaxed_counters(4);
        assert_eq!(c1, 40); // 4 threads * 10 increments
        assert_eq!(c2, 10); // 2 threads * 5 increments
    }
}
