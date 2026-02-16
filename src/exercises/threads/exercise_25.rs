//! Exercise 25: Thread-Local Storage
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use thread_local! macro
//! - Understand thread-local variables
//! - Manage per-thread state

use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;

thread_local! {
    static THREAD_COUNTER: RefCell<usize> = RefCell::new(0);
}

/// Each thread increments its thread-local counter n times.
/// Return sum of all thread-local final values.
pub fn thread_local_sum(n_threads: usize, increments_per_thread: usize) -> usize {
    let (tx, rx) = mpsc::channel();

    let handles: Vec<_> = (0..n_threads)
        .map(|_| {
            let tx = tx.clone();
            thread::spawn(move || {
                THREAD_COUNTER.with(|counter| {
                    for _ in 0..increments_per_thread {
                        *counter.borrow_mut() += 1;
                    }
                    tx.send(*counter.borrow()).unwrap();
                });
            })
        })
        .collect();

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    rx.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_local_sum() {
        assert_eq!(thread_local_sum(4, 25), 100);
        assert_eq!(thread_local_sum(10, 10), 100);
    }

    #[test]
    fn test_thread_local_sum_single_thread() {
        assert_eq!(thread_local_sum(1, 100), 100);
    }

    #[test]
    fn test_thread_local_sum_no_increments() {
        assert_eq!(thread_local_sum(5, 0), 0);
    }
}
