//! Exercise 19: RwLock with Multiple Writers
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Coordinate multiple readers and writers
//! - Understand write lock exclusivity
//! - Balance read/write access

use std::sync::{Arc, RwLock};
use std::thread;

/// Multiple threads read and write to shared data.
/// n_readers threads read, n_writers threads increment.
/// Return final value.
pub fn rwlock_multi_writer(n_readers: usize, n_writers: usize, increments_per_writer: usize) -> i32  {
    todo!("Return final value.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rwlock_multi_writer() {
        assert_eq!(rwlock_multi_writer(5, 2, 50), 100);
        assert_eq!(rwlock_multi_writer(10, 4, 25), 100);
    }

    #[test]
    fn test_rwlock_multi_writer_only_readers() {
        assert_eq!(rwlock_multi_writer(10, 0, 0), 0);
    }

    #[test]
    fn test_rwlock_multi_writer_only_writers() {
        assert_eq!(rwlock_multi_writer(0, 10, 10), 100);
    }
}
