//! Exercise 26: Custom Allocator Patterns - Working with allocators
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand memory allocation concepts
//! - Track allocations and deallocations
//! - Build memory-aware data structures

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);
static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

/// A wrapper around raw allocation that tracks memory usage.
pub struct TrackedAlloc<T> {
    ptr: NonNull<T>,
    layout: Layout,
}

impl<T> TrackedAlloc<T> {
    pub fn new(value: T) -> Self {
        todo!("Implement new")
    }

    pub fn get(&self) -> &T {
        todo!("Implement get")
    }

    pub fn get_mut(&mut self) -> &mut T {
        todo!("Implement get_mut")
    }
}

impl<T> Drop for TrackedAlloc<T> {
    fn drop(&mut self) {
        todo!("Implement drop")
    }
}

/// Get total allocated bytes.
pub fn get_allocated_bytes() -> usize {
    todo!("Implement get_allocated_bytes")
}

/// Get total allocation count.
pub fn get_allocation_count() -> usize {
    todo!("Implement get_allocation_count")
}

/// Reset allocation tracking.
pub fn reset_allocation_tracking() {
    todo!("Implement reset_allocation_tracking")
}

/// A pool allocator concept (simplified).
pub struct MemoryPool {
    capacity: usize,
    used: usize,
}

impl MemoryPool {
    pub fn new(capacity: usize) -> Self {
        todo!("Implement new")
    }

    pub fn allocate(&mut self, size: usize) -> Result<usize, &'static str> {
        todo!("Implement allocate")
    }

    pub fn free(&mut self, size: usize) {
        todo!("Implement free")
    }

    pub fn available(&self) -> usize {
        todo!("Implement available")
    }

    pub fn used(&self) -> usize {
        todo!("Implement used")
    }

    pub fn reset(&mut self) {
        todo!("Implement reset")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracked_alloc() {
        reset_allocation_tracking();
        {
            let tracked = TrackedAlloc::new(42i32);
            assert_eq!(*tracked.get(), 42);
            assert!(get_allocated_bytes() >= std::mem::size_of::<i32>());
            assert_eq!(get_allocation_count(), 1);
        }
        // Memory freed after drop
    }

    #[test]
    fn test_tracked_alloc_mut() {
        reset_allocation_tracking();
        let mut tracked = TrackedAlloc::new(10);
        *tracked.get_mut() = 20;
        assert_eq!(*tracked.get(), 20);
    }

    #[test]
    fn test_multiple_allocations() {
        reset_allocation_tracking();
        let _a1 = TrackedAlloc::new(1u8);
        let _a2 = TrackedAlloc::new(2u16);
        let _a3 = TrackedAlloc::new(3u32);
        
        assert_eq!(get_allocation_count(), 3);
        assert!(get_allocated_bytes() >= 7); // at least 1 + 2 + 4 bytes
    }

    #[test]
    fn test_memory_pool_allocate() {
        let mut pool = MemoryPool::new(100);
        assert_eq!(pool.available(), 100);
        
        let offset1 = pool.allocate(30).unwrap();
        assert_eq!(offset1, 0);
        assert_eq!(pool.used(), 30);
        assert_eq!(pool.available(), 70);
        
        let offset2 = pool.allocate(40).unwrap();
        assert_eq!(offset2, 30);
        assert_eq!(pool.used(), 70);
    }

    #[test]
    fn test_memory_pool_out_of_memory() {
        let mut pool = MemoryPool::new(50);
        assert!(pool.allocate(30).is_ok());
        assert!(pool.allocate(30).is_err());
    }

    #[test]
    fn test_memory_pool_free() {
        let mut pool = MemoryPool::new(100);
        pool.allocate(50).unwrap();
        assert_eq!(pool.used(), 50);
        
        pool.free(20);
        assert_eq!(pool.used(), 30);
        assert_eq!(pool.available(), 70);
    }

    #[test]
    fn test_memory_pool_reset() {
        let mut pool = MemoryPool::new(100);
        pool.allocate(50).unwrap();
        pool.allocate(30).unwrap();
        assert_eq!(pool.used(), 80);
        
        pool.reset();
        assert_eq!(pool.used(), 0);
        assert_eq!(pool.available(), 100);
    }

    #[test]
    fn test_tracked_alloc_with_string() {
        reset_allocation_tracking();
        let tracked = TrackedAlloc::new(String::from("test"));
        assert_eq!(tracked.get(), "test");
        assert!(get_allocated_bytes() > 0);
    }
}
