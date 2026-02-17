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
    let data = Arc::new(RwLock::new(value));
    
    // Spawn readers
    let reader_handles: Vec<_> = (0..n_readers)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let _val = *data.read().unwrap();
            })
        })
        .collect();

    // Spawn one writer
    let data_clone = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut val = data_clone.write().unwrap();
        *val += 10;
    });

    for handle in reader_handles {
        handle.join().unwrap();
    }
    writer_handle.join().unwrap();

    *data.read().unwrap()
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
