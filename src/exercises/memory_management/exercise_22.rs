//! Exercise 22: Drop Trait Basics - Implementing custom cleanup
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement the Drop trait
//! - Understand resource cleanup
//! - Learn RAII pattern basics

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct DropCounter {
    name: String,
    counter: Arc<AtomicUsize>,
}

impl DropCounter {
    pub fn new(name: String, counter: Arc<AtomicUsize>) -> Self {
        DropCounter { name, counter }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct Resource {
    id: u32,
    data: Vec<u8>,
}

impl Resource {
    pub fn new(id: u32, size: usize) -> Self {
        Resource {
            id,
            data: vec![0; size],
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        // Simulate cleanup
        self.data.clear();
    }
}

/// Create and drop a resource, returning its ID.
pub fn use_resource(id: u32) -> u32 {
    let resource = Resource::new(id, 1024);
    resource.id()
    // resource is dropped here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_counter() {
        let counter = Arc::new(AtomicUsize::new(0));
        {
            let _dc1 = DropCounter::new(String::from("first"), counter.clone());
            let _dc2 = DropCounter::new(String::from("second"), counter.clone());
        }
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_resource_creation() {
        let resource = Resource::new(1, 100);
        assert_eq!(resource.id(), 1);
        assert_eq!(resource.size(), 100);
    }

    #[test]
    fn test_use_resource() {
        let id = use_resource(42);
        assert_eq!(id, 42);
    }

    #[test]
    fn test_multiple_drops() {
        let counter = Arc::new(AtomicUsize::new(0));
        
        let dc1 = DropCounter::new(String::from("test1"), counter.clone());
        drop(dc1);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        
        let dc2 = DropCounter::new(String::from("test2"), counter.clone());
        drop(dc2);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_drop_order() {
        let counter = Arc::new(AtomicUsize::new(0));
        {
            let _dc1 = DropCounter::new(String::from("first"), counter.clone());
            {
                let _dc2 = DropCounter::new(String::from("second"), counter.clone());
            }
            // dc2 dropped first
            assert_eq!(counter.load(Ordering::SeqCst), 1);
        }
        // dc1 dropped second
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
}
