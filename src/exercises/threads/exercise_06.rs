//! Exercise 06: Thread Result Handling
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Handle thread panics with join()
//! - Understand Result from JoinHandle
//! - Work with thread::Result

use std::thread;

/// Spawn a thread that panics if value is negative, otherwise returns value * 2.
/// Return Ok with the result, or Err if the thread panicked.
pub fn safe_compute(value: i32) -> Result<i32, String> {
    todo!("Implement safe_compute")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_compute_success() {
        assert_eq!(safe_compute(5), Ok(10));
        assert_eq!(safe_compute(0), Ok(0));
        assert_eq!(safe_compute(100), Ok(200));
    }

    #[test]
    fn test_safe_compute_panic() {
        assert!(safe_compute(-1).is_err());
        assert!(safe_compute(-10).is_err());
    }
}
