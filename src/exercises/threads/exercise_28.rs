//! Exercise 28: Acquire-Release Ordering
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand Acquire-Release semantics
//! - Implement happens-before relationships
//! - Synchronize without locks

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Writer thread stores data then sets flag with Release.
/// Reader thread waits for flag with Acquire, then reads data.
pub fn acquire_release_pattern(value: usize) -> usize {
    let data = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let data_clone = Arc::clone(&data);
    let ready_clone = Arc::clone(&ready);

    let writer = thread::spawn(move || {
        data_clone.store(value, Ordering::Relaxed);
        ready_clone.store(true, Ordering::Release);
    });

    let reader = thread::spawn(move || {
        while !ready.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        data.load(Ordering::Relaxed)
    });

    writer.join().unwrap();
    reader.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_release_pattern() {
        assert_eq!(acquire_release_pattern(42), 42);
        assert_eq!(acquire_release_pattern(1000), 1000);
        assert_eq!(acquire_release_pattern(0), 0);
    }

    #[test]
    fn test_acquire_release_multiple_runs() {
        for i in 0..10 {
            assert_eq!(acquire_release_pattern(i * 10), i * 10);
        }
    }
}
