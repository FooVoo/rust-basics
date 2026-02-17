//! Exercise 16: Scoped Threads
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use thread::scope for borrowing
//! - Understand scoped thread lifetimes
//! - Avoid unnecessary cloning

use std::thread;

/// Use scoped threads to increment each element of a slice in parallel.
pub fn parallel_increment(data: &mut [i32], n_threads: usize)  {
    todo!("Use scoped threads to increment each element of a slice in parallel.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_increment() {
        let mut data = vec![1, 2, 3, 4, 5];
        parallel_increment(&mut data, 2);
        assert_eq!(data, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_parallel_increment_single_thread() {
        let mut data = vec![10, 20, 30];
        parallel_increment(&mut data, 1);
        assert_eq!(data, vec![11, 21, 31]);
    }

    #[test]
    fn test_parallel_increment_empty() {
        let mut data: Vec<i32> = vec![];
        parallel_increment(&mut data, 4);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_parallel_increment_many_threads() {
        let mut data = vec![0; 100];
        parallel_increment(&mut data, 10);
        assert!(data.iter().all(|&x| x == 1));
    }
}
