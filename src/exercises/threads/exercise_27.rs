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
    let counter1 = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..n_threads)
        .map(|i| {
            let counter1 = Arc::clone(&counter1);
            let counter2 = Arc::clone(&counter2);
            thread::spawn(move || {
                // Increment counter1 with Relaxed (independent operations)
                for _ in 0..10 {
                    counter1.fetch_add(1, Ordering::Relaxed);
                }
                
                // Increment counter2 with Relaxed (independent operations)
                if i % 2 == 0 {
                    for _ in 0..5 {
                        counter2.fetch_add(1, Ordering::Relaxed);
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    (
        counter1.load(Ordering::SeqCst),
        counter2.load(Ordering::SeqCst),
    )
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
