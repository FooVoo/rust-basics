//! Exercise 23: Custom Smart Pointer with Drop - Implement Drop trait
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement Drop trait for cleanup
//! - Understand drop order and timing
//! - Create RAII wrappers

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);
static DROP_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// A guard that logs when dropped.
pub struct Guard {
    name: String,
}

impl Guard {
    pub fn new(name: String) -> Self  {
        todo!("A guard that logs when dropped.")
    }

    pub fn name(&self) -> &str  {
        todo!("Implement name")
    }
}

impl Drop for Guard {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

/// Reset global counters for testing.
pub fn reset_drop_tracking()  {
    todo!("Reset global counters for testing.")
}

/// Get total drop count.
pub fn get_drop_count() -> usize  {
    todo!("Get total drop count.")
}

/// Get drop log.
pub fn get_drop_log() -> Vec<String>  {
    todo!("Get drop log.")
}

/// A simple RAII file handle simulator.
pub struct FileHandle {
    filename: String,
    closed: std::cell::Cell<bool>,
}

impl FileHandle {
    pub fn open(filename: String) -> Self  {
        todo!("A simple RAII file handle simulator.")
    }

    pub fn is_closed(&self) -> bool  {
        todo!("Implement is_closed")
    }

    pub fn close(&self)  {
        todo!("Implement close")
    }
}

impl Drop for FileHandle {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_drop() {
        reset_drop_tracking();
        {
            let _g = Guard::new("test".to_string());
        }
        assert_eq!(get_drop_count(), 1);
    }

    #[test]
    fn test_multiple_guards() {
        reset_drop_tracking();
        {
            let _g1 = Guard::new("first".to_string());
            let _g2 = Guard::new("second".to_string());
            let _g3 = Guard::new("third".to_string());
        }
        assert_eq!(get_drop_count(), 3);
    }

    #[test]
    fn test_drop_order() {
        reset_drop_tracking();
        {
            let _g1 = Guard::new("first".to_string());
            let _g2 = Guard::new("second".to_string());
        }
        let log = get_drop_log();
        // Drop in reverse order of creation
        assert_eq!(log, vec!["second", "first"]);
    }

    #[test]
    fn test_nested_drops() {
        reset_drop_tracking();
        {
            let _g1 = Guard::new("outer".to_string());
            {
                let _g2 = Guard::new("inner".to_string());
            }
        }
        let log = get_drop_log();
        assert_eq!(log, vec!["inner", "outer"]);
    }

    #[test]
    fn test_file_handle_auto_close() {
        let handle = FileHandle::open("test.txt".to_string());
        assert!(!handle.is_closed());
        drop(handle);
        // File is automatically closed
    }

    #[test]
    fn test_file_handle_manual_close() {
        let handle = FileHandle::open("test.txt".to_string());
        handle.close();
        assert!(handle.is_closed());
    }

    #[test]
    fn test_early_drop() {
        reset_drop_tracking();
        let g = Guard::new("early".to_string());
        assert_eq!(get_drop_count(), 0);
        drop(g);
        assert_eq!(get_drop_count(), 1);
    }

    #[test]
    fn test_guard_name() {
        let g = Guard::new("test_name".to_string());
        assert_eq!(g.name(), "test_name");
    }
}
