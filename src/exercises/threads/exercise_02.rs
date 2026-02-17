//! Exercise 02: Multiple Threads
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Spawn multiple threads
//! - Collect results from multiple threads
//! - Work with JoinHandle

use std::thread;

/// Spawn N threads, each computing i*i for i in 0..n.
/// Return a vector with all results in order.
pub fn parallel_squares(n: usize) -> Vec<usize> {
    todo!("Implement parallel_squares")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_squares() {
        assert_eq!(parallel_squares(0), vec![]);
        assert_eq!(parallel_squares(1), vec![0]);
        assert_eq!(parallel_squares(5), vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_parallel_squares_larger() {
        let result = parallel_squares(10);
        assert_eq!(result.len(), 10);
        assert_eq!(result[9], 81);
    }
}
