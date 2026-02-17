//! Exercise 20: Thread Parking
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use thread::park and unpark
//! - Understand thread signaling
//! - Coordinate threads without channels

use std::thread;
use std::time::Duration;

/// Park the current thread until unparked by another thread.
/// Return true if unparked within timeout.
pub fn park_with_timeout(timeout_ms: u64) -> bool {
    let handle = thread::current();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(timeout_ms / 2));
        handle.unpark();
    });

    let start = std::time::Instant::now();
    thread::park_timeout(Duration::from_millis(timeout_ms));
    start.elapsed() < Duration::from_millis(timeout_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park_with_timeout_unparked() {
        assert!(park_with_timeout(100));
    }

    #[test]
    fn test_unpark_before_park() {
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_millis(10));
            thread::park();
        });
        
        thread::sleep(Duration::from_millis(5));
        handle.thread().unpark();
        handle.join().unwrap();
    }
}
