//! Exercise 15: Parallel Sum
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement parallel reduction
//! - Divide and conquer pattern
//! - Aggregate results from threads

use std::thread;

/// Sum a vector of numbers in parallel using n_threads.
pub fn parallel_sum(numbers: Vec<i64>, n_threads: usize) -> i64  {
    todo!("Sum a vector of numbers in parallel using n_threads.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        assert_eq!(parallel_sum(vec![1, 2, 3, 4, 5], 2), 15);
        assert_eq!(parallel_sum(vec![10, 20, 30], 2), 60);
    }

    #[test]
    fn test_parallel_sum_single_thread() {
        assert_eq!(parallel_sum(vec![1, 2, 3, 4, 5], 1), 15);
    }

    #[test]
    fn test_parallel_sum_empty() {
        assert_eq!(parallel_sum(vec![], 4), 0);
    }

    #[test]
    fn test_parallel_sum_many_threads() {
        let numbers: Vec<i64> = (1..=100).collect();
        assert_eq!(parallel_sum(numbers, 10), 5050);
    }
}
