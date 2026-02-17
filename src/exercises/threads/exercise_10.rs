//! Exercise 10: Shared Vector with Mutex
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Protect collections with Mutex
//! - Share complex data between threads
//! - Manage mutable shared state

use std::sync::{Arc, Mutex};
use std::thread;

/// Multiple threads push their thread indices to a shared vector.
/// Return the sorted vector.
pub fn concurrent_vec_push(n_threads: usize) -> Vec<usize> {
    todo!("Implement concurrent_vec_push")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_vec_push() {
        assert_eq!(concurrent_vec_push(0), vec![]);
        assert_eq!(concurrent_vec_push(1), vec![0]);
        assert_eq!(concurrent_vec_push(5), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_concurrent_vec_push_larger() {
        let result = concurrent_vec_push(10);
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
