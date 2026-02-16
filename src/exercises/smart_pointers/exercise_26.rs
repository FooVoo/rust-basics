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
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr.write(value);
            
            ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::SeqCst);
            ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
            
            TrackedAlloc {
                ptr: NonNull::new_unchecked(ptr),
                layout,
            }
        }
    }

    pub fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for TrackedAlloc<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
            dealloc(self.ptr.as_ptr() as *mut u8, self.layout);
            ALLOCATED_BYTES.fetch_sub(self.layout.size(), Ordering::SeqCst);
        }
    }
}

/// Get total allocated bytes.
pub fn get_allocated_bytes() -> usize {
    ALLOCATED_BYTES.load(Ordering::SeqCst)
}

/// Get total allocation count.
pub fn get_allocation_count() -> usize {
    ALLOCATION_COUNT.load(Ordering::SeqCst)
}

/// Reset allocation tracking.
pub fn reset_allocation_tracking() {
    ALLOCATED_BYTES.store(0, Ordering::SeqCst);
    ALLOCATION_COUNT.store(0, Ordering::SeqCst);
}

/// A pool allocator concept (simplified).
pub struct MemoryPool {
    capacity: usize,
    used: usize,
}

impl MemoryPool {
    pub fn new(capacity: usize) -> Self {
        MemoryPool { capacity, used: 0 }
    }

    pub fn allocate(&mut self, size: usize) -> Result<usize, &'static str> {
        if self.used + size <= self.capacity {
            let offset = self.used;
            self.used += size;
            Ok(offset)
        } else {
            Err("Out of memory")
        }
    }

    pub fn free(&mut self, size: usize) {
        self.used = self.used.saturating_sub(size);
    }

    pub fn available(&self) -> usize {
        self.capacity - self.used
    }

    pub fn used(&self) -> usize {
        self.used
    }

    pub fn reset(&mut self) {
        self.used = 0;
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
