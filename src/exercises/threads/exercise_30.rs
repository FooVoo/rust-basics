//! Exercise 30: Seqlock (Sequence Lock)
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement seqlock for read-heavy workloads
//! - Use sequence numbers for consistency
//! - Understand optimistic concurrency
//! - Master memory ordering in complex scenarios

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub struct SeqLock<T> {
    seq: AtomicUsize,
    data: std::cell::UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SeqLock<T> {}
unsafe impl<T: Send> Sync for SeqLock<T> {}

impl<T: Copy> SeqLock<T> {
    pub fn new(data: T) -> Self {
        todo!("Implement new")
    }

    pub fn read(&self) -> T {
        todo!("Implement read")
    }

    pub fn write(&self, data: T) {
        todo!("Implement write")
    }
}

/// Test seqlock with multiple readers and one writer.
pub fn test_seqlock(n_readers: usize, n_writes: usize) -> Vec<i32> {
    todo!("Implement test_seqlock")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seqlock_basic() {
        let seqlock = SeqLock::new(42);
        assert_eq!(seqlock.read(), 42);
        
        seqlock.write(100);
        assert_eq!(seqlock.read(), 100);
    }

    #[test]
    fn test_seqlock_concurrent() {
        let result = test_seqlock(10, 50);
        assert_eq!(result[0], 49);
    }

    #[test]
    fn test_seqlock_many_readers() {
        let result = test_seqlock(20, 30);
        assert_eq!(result[0], 29);
    }

    #[test]
    fn test_seqlock_multiple_writes() {
        let seqlock = SeqLock::new(0);
        for i in 1..=10 {
            seqlock.write(i);
        }
        assert_eq!(seqlock.read(), 10);
    }
}
