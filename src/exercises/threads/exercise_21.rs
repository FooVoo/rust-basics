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
    let counter = Arc::new(Mutex::new(0));
    let barrier1 = Arc::new(Barrier::new(n_threads));
    let barrier2 = Arc::new(Barrier::new(n_threads));
    let phase1_result = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..n_threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            let barrier1 = Arc::clone(&barrier1);
            let barrier2 = Arc::clone(&barrier2);
            let phase1_result = Arc::clone(&phase1_result);
            
            thread::spawn(move || {
                // Phase 1
                {
                    let mut c = counter.lock().unwrap();
                    *c += 1;
                }
                
                // Wait at barrier after phase 1
                let wait_result = barrier1.wait();
                
                // Record phase 1 result (only leader thread)
                if wait_result.is_leader() {
                    *phase1_result.lock().unwrap() = *counter.lock().unwrap();
                }
                
                // Wait for all threads to pass the recording point
                barrier2.wait();
                
                // Phase 2
                {
                    let mut c = counter.lock().unwrap();
                    *c += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    (*phase1_result.lock().unwrap(), *counter.lock().unwrap())
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
