//! Exercise 07: Box Drop - Understand automatic cleanup
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Drop trait behavior with Box
//! - Learn about automatic resource cleanup
//! - Track allocation and deallocation

use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Tracked {
    pub id: usize,
}

impl Tracked {
    pub fn new(id: usize) -> Self {
        ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
        Tracked { id }
    }
}

impl Drop for Tracked {
    fn drop(&mut self) {
        DEALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

/// Get current allocation count.
pub fn allocations() -> usize {
    ALLOCATION_COUNT.load(Ordering::SeqCst)
}

/// Get current deallocation count.
pub fn deallocations() -> usize {
    DEALLOCATION_COUNT.load(Ordering::SeqCst)
}

/// Reset counters.
pub fn reset_counters() {
    ALLOCATION_COUNT.store(0, Ordering::SeqCst);
    DEALLOCATION_COUNT.store(0, Ordering::SeqCst);
}

/// Create and immediately drop a boxed value.
pub fn create_and_drop() {
    let _boxed = Box::new(Tracked::new(1));
    // Automatically dropped here
}

/// Create multiple boxed values in a vector.
pub fn create_multiple(count: usize) -> Vec<Box<Tracked>> {
    (0..count).map(|i| Box::new(Tracked::new(i))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_drop() {
        reset_counters();
        {
            let _b = Box::new(Tracked::new(1));
            assert_eq!(allocations(), 1);
            assert_eq!(deallocations(), 0);
        }
        // Box dropped here
        assert_eq!(deallocations(), 1);
    }

    #[test]
    fn test_create_and_drop() {
        reset_counters();
        create_and_drop();
        assert_eq!(allocations(), 1);
        assert_eq!(deallocations(), 1);
    }

    #[test]
    fn test_multiple_boxes() {
        reset_counters();
        {
            let boxes = create_multiple(5);
            assert_eq!(allocations(), 5);
            assert_eq!(deallocations(), 0);
            assert_eq!(boxes.len(), 5);
        }
        // All boxes dropped here
        assert_eq!(deallocations(), 5);
    }

    #[test]
    fn test_early_drop() {
        reset_counters();
        let b = Box::new(Tracked::new(1));
        assert_eq!(deallocations(), 0);
        drop(b);
        assert_eq!(deallocations(), 1);
    }
}
