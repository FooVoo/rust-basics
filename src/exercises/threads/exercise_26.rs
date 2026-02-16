//! Exercise 26: Atomic Flags and Spin Locks
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement spinlock with atomics
//! - Use AtomicBool for flags
//! - Understand busy-waiting

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            while self.locked.load(Ordering::Relaxed) {
                std::hint::spin_loop();
            }
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

/// Use a spinlock to protect a counter.
pub fn spinlock_counter(n_threads: usize, increments_per_thread: usize) -> usize {
    let lock = Arc::new(SpinLock::new());
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..n_threads)
        .map(|_| {
            let lock = Arc::clone(&lock);
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments_per_thread {
                    lock.lock();
                    let val = counter.load(Ordering::Relaxed);
                    counter.store(val + 1, Ordering::Relaxed);
                    lock.unlock();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock_counter() {
        assert_eq!(spinlock_counter(4, 250), 1000);
        assert_eq!(spinlock_counter(10, 100), 1000);
    }

    #[test]
    fn test_spinlock_counter_single_thread() {
        assert_eq!(spinlock_counter(1, 500), 500);
    }
}
