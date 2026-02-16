//! Exercise 22: Condition Variable
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use Condvar for complex synchronization
//! - Implement wait/notify patterns
//! - Handle spurious wakeups

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/// Producer sets value when ready, consumer waits via condvar.
pub fn condvar_pattern(value: i32) -> i32 {
    let pair = Arc::new((Mutex::new((false, 0)), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut data = lock.lock().unwrap();
        
        while !data.0 {
            data = cvar.wait(data).unwrap();
        }
        
        data.1
    });

    let producer = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut data = lock.lock().unwrap();
        data.0 = true;
        data.1 = value;
        cvar.notify_one();
    });

    producer.join().unwrap();
    consumer.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condvar_pattern() {
        assert_eq!(condvar_pattern(42), 42);
        assert_eq!(condvar_pattern(100), 100);
        assert_eq!(condvar_pattern(-5), -5);
    }

    #[test]
    fn test_condvar_pattern_zero() {
        assert_eq!(condvar_pattern(0), 0);
    }
}
