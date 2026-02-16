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
        SeqLock {
            seq: AtomicUsize::new(0),
            data: std::cell::UnsafeCell::new(data),
        }
    }

    pub fn read(&self) -> T {
        loop {
            let seq1 = self.seq.load(Ordering::Acquire);
            
            // If sequence is odd, a write is in progress
            if seq1 % 2 == 1 {
                std::hint::spin_loop();
                continue;
            }

            // Read data
            let data = unsafe { *self.data.get() };

            // Check sequence hasn't changed
            let seq2 = self.seq.load(Ordering::Acquire);
            if seq1 == seq2 {
                return data;
            }
        }
    }

    pub fn write(&self, data: T) {
        // Increment sequence to odd (write in progress)
        let seq = self.seq.fetch_add(1, Ordering::Acquire);
        
        // Ensure we started from even
        assert_eq!(seq % 2, 0, "Concurrent writes detected!");

        // Write data
        unsafe {
            *self.data.get() = data;
        }

        // Increment sequence to even (write complete)
        self.seq.fetch_add(1, Ordering::Release);
    }
}

/// Test seqlock with multiple readers and one writer.
pub fn test_seqlock(n_readers: usize, n_writes: usize) -> Vec<i32> {
    let seqlock = Arc::new(SeqLock::new(0));
    let mut handles = vec![];

    // Spawn readers
    for _ in 0..n_readers {
        let seqlock = Arc::clone(&seqlock);
        handles.push(thread::spawn(move || {
            let mut last_seen = -1;
            for _ in 0..100 {
                let val = seqlock.read();
                // Values should be monotonically increasing
                assert!(val >= last_seen);
                last_seen = val;
            }
        }));
    }

    // Spawn writer
    let seqlock_writer = Arc::clone(&seqlock);
    let writer = thread::spawn(move || {
        for i in 0..n_writes {
            seqlock_writer.write(i as i32);
            thread::yield_now();
        }
    });

    writer.join().unwrap();

    for handle in handles {
        handle.join().unwrap();
    }

    // Collect final values
    vec![seqlock.read()]
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
