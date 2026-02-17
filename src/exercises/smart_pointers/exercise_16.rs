//! Exercise 16: Arc + Mutex Pattern - Thread-safe shared mutable state
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Combine Arc and Mutex for thread-safe mutation
//! - Understand the Arc+Mutex pattern
//! - Work with lock() and MutexGuard

use std::sync::{Arc, Mutex};

/// Create a shared mutable counter.
pub fn create_shared_counter(initial: i32) -> Arc<Mutex<i32>>  {
    todo!("Create a shared mutable counter.")
}

/// Increment a shared counter.
pub fn increment_counter(counter: &Arc<Mutex<i32>>)  {
    todo!("Increment a shared counter.")
}

/// Get the current counter value.
pub fn get_counter_value(counter: &Arc<Mutex<i32>>) -> i32  {
    todo!("Get the current counter value.")
}

/// A thread-safe accumulator.
pub struct Accumulator {
    value: Arc<Mutex<i32>>,
}

impl Accumulator {
    pub fn new() -> Self  {
        todo!("A thread-safe accumulator.")
    }

    pub fn add(&self, n: i32)  {
        todo!("Implement add")
    }

    pub fn get(&self) -> i32  {
        todo!("Implement get")
    }

    pub fn reset(&self)  {
        todo!("Implement reset")
    }

    pub fn clone_handle(&self) -> Self  {
        todo!("Implement clone_handle")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shared_counter() {
        let counter = create_shared_counter(0);
        assert_eq!(get_counter_value(&counter), 0);
    }

    #[test]
    fn test_increment_counter() {
        let counter = create_shared_counter(10);
        increment_counter(&counter);
        assert_eq!(get_counter_value(&counter), 11);
        increment_counter(&counter);
        assert_eq!(get_counter_value(&counter), 12);
    }

    #[test]
    fn test_multiple_references() {
        let counter = create_shared_counter(0);
        let counter2 = Arc::clone(&counter);
        
        increment_counter(&counter);
        increment_counter(&counter2);
        
        assert_eq!(get_counter_value(&counter), 2);
        assert_eq!(get_counter_value(&counter2), 2);
    }

    #[test]
    fn test_accumulator() {
        let acc = Accumulator::new();
        assert_eq!(acc.get(), 0);
        
        acc.add(5);
        assert_eq!(acc.get(), 5);
        
        acc.add(10);
        assert_eq!(acc.get(), 15);
        
        acc.reset();
        assert_eq!(acc.get(), 0);
    }

    #[test]
    fn test_accumulator_clone() {
        let acc1 = Accumulator::new();
        let acc2 = acc1.clone_handle();
        
        acc1.add(10);
        acc2.add(20);
        
        // Both share the same underlying value
        assert_eq!(acc1.get(), 30);
        assert_eq!(acc2.get(), 30);
    }

    #[test]
    fn test_mutex_guard_drops() {
        let counter = create_shared_counter(0);
        {
            let mut guard = counter.lock().unwrap();
            *guard = 42;
            // guard dropped here
        }
        assert_eq!(get_counter_value(&counter), 42);
    }

    #[test]
    fn test_accumulator_multiple_operations() {
        let acc = Accumulator::new();
        let handles: Vec<_> = (0..5).map(|_| acc.clone_handle()).collect();
        
        for (i, handle) in handles.iter().enumerate() {
            handle.add((i + 1) as i32);
        }
        
        // 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(acc.get(), 15);
    }
}
