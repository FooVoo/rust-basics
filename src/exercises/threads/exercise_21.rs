//! Exercise 21: Barrier Synchronization
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use Barrier for thread synchronization
//! - Coordinate phases of execution
//! - Implement multi-phase algorithms

use std::sync::{Arc, Barrier, Mutex};
use std::thread;

/// Each thread increments a counter, waits at barrier, then increments again.
/// Return the counter value after first phase and final value.
pub fn barrier_phases(n_threads: usize) -> (usize, usize) {
    todo!("Implement barrier_phases")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier_phases() {
        let (phase1, final_val) = barrier_phases(5);
        assert_eq!(phase1, 5);
        assert_eq!(final_val, 10);
    }

    #[test]
    fn test_barrier_phases_single_thread() {
        let (phase1, final_val) = barrier_phases(1);
        assert_eq!(phase1, 1);
        assert_eq!(final_val, 2);
    }

    #[test]
    fn test_barrier_phases_many() {
        let (phase1, final_val) = barrier_phases(10);
        assert_eq!(phase1, 10);
        assert_eq!(final_val, 20);
    }
}
