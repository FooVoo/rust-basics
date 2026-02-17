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
pub fn rwlock_multi_writer(n_readers: usize, n_writers: usize, increments_per_writer: usize) -> i32 {
    let data = Arc::new(RwLock::new(0));

    // Spawn readers
    let reader_handles: Vec<_> = (0..n_readers)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let _val = *data.read().unwrap();
            })
        })
        .collect();

    // Spawn writers
    let writer_handles: Vec<_> = (0..n_writers)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                for _ in 0..increments_per_writer {
                    let mut val = data.write().unwrap();
                    *val += 1;
                }
            })
        })
        .collect();

    for handle in reader_handles.into_iter().chain(writer_handles) {
        handle.join().unwrap();
    }

    *data.read().unwrap()
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
