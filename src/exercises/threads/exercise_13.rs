//! Exercise 13: Thread Pool Concept
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand thread pool pattern
//! - Distribute work across fixed threads
//! - Use channels for work queue

use std::sync::mpsc;
use std::thread;

/// Simulate a simple thread pool: spawn n_workers threads.
/// Send n_tasks work items through a channel, each worker processes tasks.
/// Return sum of all task results (each task returns its index * 2).
pub fn simple_thread_pool(n_workers: usize, n_tasks: usize) -> usize {
    todo!("Implement simple_thread_pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_thread_pool() {
        // 2 workers, 5 tasks: (0+1+2+3+4) * 2 = 20
        assert_eq!(simple_thread_pool(2, 5), 20);
        // 4 workers, 10 tasks: (0+1+...+9) * 2 = 90
        assert_eq!(simple_thread_pool(4, 10), 90);
    }

    #[test]
    fn test_simple_thread_pool_edge_cases() {
        assert_eq!(simple_thread_pool(1, 0), 0);
        assert_eq!(simple_thread_pool(5, 1), 0);
    }
}
