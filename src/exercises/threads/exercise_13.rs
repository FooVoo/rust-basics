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
    let (tx, rx) = mpsc::channel();
    let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));

    // Spawn workers
    let sum = std::sync::Arc::new(std::sync::Mutex::new(0));
    let handles: Vec<_> = (0..n_workers)
        .map(|_| {
            let rx = std::sync::Arc::clone(&rx);
            let sum = std::sync::Arc::clone(&sum);
            thread::spawn(move || {
                while let Ok(task) = rx.lock().unwrap().recv() {
                    let result: usize = task;
                    *sum.lock().unwrap() += result * 2;
                }
            })
        })
        .collect();

    // Send tasks
    for i in 0..n_tasks {
        tx.send(i).unwrap();
    }
    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    *sum.lock().unwrap()
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
