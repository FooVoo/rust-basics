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
    fn new() -> Self  {
        todo!("Implement new")
    }

    fn lock(&self)  {
        todo!("Implement lock")
    }

    fn unlock(&self)  {
        todo!("Implement unlock")
    }
}

/// Use a spinlock to protect a counter.
pub fn spinlock_counter(n_threads: usize, increments_per_thread: usize) -> usize  {
    todo!("Use a spinlock to protect a counter.")
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
