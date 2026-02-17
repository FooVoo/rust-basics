//! Exercise 12: Basic RwLock
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand RwLock for read-heavy workloads
//! - Distinguish read vs write locks
//! - Use RwLock effectively

use std::sync::{Arc, RwLock};
use std::thread;

/// Multiple threads read a value, one thread writes.
/// Return the final value after all operations.
pub fn rwlock_operations(n_readers: usize, value: i32) -> i32 {
    todo!("Implement rwlock_operations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rwlock_operations() {
        assert_eq!(rwlock_operations(0, 5), 15);
        assert_eq!(rwlock_operations(5, 10), 20);
        assert_eq!(rwlock_operations(10, 0), 10);
    }

    #[test]
    fn test_rwlock_operations_negative() {
        assert_eq!(rwlock_operations(3, -5), 5);
    }
}
