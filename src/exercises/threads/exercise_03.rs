//! Exercise 03: Thread with Sleep
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use thread::sleep
//! - Understand thread timing
//! - Work with Duration

use std::thread;
use std::time::Duration;

/// Spawn a thread that sleeps for the given milliseconds, then returns the value.
pub fn sleep_and_return(millis: u64, value: i32) -> i32  {
    todo!("Spawn a thread that sleeps for the given milliseconds, then returns the value.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_sleep_and_return() {
        assert_eq!(sleep_and_return(10, 42), 42);
        assert_eq!(sleep_and_return(5, 100), 100);
    }

    #[test]
    fn test_sleep_duration() {
        let start = Instant::now();
        sleep_and_return(50, 0);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(50));
        assert!(elapsed < Duration::from_millis(200)); // Allow some overhead
    }
}
