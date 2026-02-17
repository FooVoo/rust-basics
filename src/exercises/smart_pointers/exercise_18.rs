//! Exercise 18: Smart Pointer Patterns - Common patterns and idioms
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Learn common smart pointer patterns
//! - Understand when to use each type
//! - Build practical data structures

use std::cell::RefCell;
use std::rc::Rc;

/// A simple cache using Rc for shared immutable data.
pub struct ImmutableCache {
    entries: Vec<Rc<String>>,
}

impl ImmutableCache {
    pub fn new() -> Self  {
        todo!("A simple cache using Rc for shared immutable data.")
    }

    pub fn add(&mut self, value: String) -> Rc<String>  {
        todo!("Implement add")
    }

    pub fn get(&self, index: usize) -> Option<Rc<String>>  {
        todo!("Implement get")
    }

    pub fn len(&self) -> usize  {
        todo!("Implement len")
    }
}

/// A mutable cache using Rc+RefCell.
pub struct MutableCache {
    entries: Rc<RefCell<Vec<String>>>,
}

impl MutableCache {
    pub fn new() -> Self  {
        todo!("A mutable cache using Rc+RefCell.")
    }

    pub fn add(&self, value: String)  {
        todo!("Implement add")
    }

    pub fn get(&self, index: usize) -> Option<String>  {
        todo!("Implement get")
    }

    pub fn len(&self) -> usize  {
        todo!("Implement len")
    }

    pub fn clone_handle(&self) -> Self  {
        todo!("Implement clone_handle")
    }
}

/// Observable value pattern.
pub struct Observable<T> {
    value: Rc<RefCell<T>>,
}

impl<T: Clone> Observable<T> {
    pub fn new(value: T) -> Self  {
        todo!("Observable value pattern.")
    }

    pub fn get(&self) -> T  {
        todo!("Implement get")
    }

    pub fn set(&self, value: T)  {
        todo!("Implement set")
    }

    pub fn subscribe(&self) -> Self  {
        todo!("Implement subscribe")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_cache() {
        let mut cache = ImmutableCache::new();
        
        let rc1 = cache.add("first".to_string());
        let rc2 = cache.add("second".to_string());
        
        assert_eq!(cache.len(), 2);
        assert_eq!(*rc1, "first");
        assert_eq!(*rc2, "second");
        
        let retrieved = cache.get(0).unwrap();
        assert_eq!(*retrieved, "first");
    }

    #[test]
    fn test_immutable_cache_sharing() {
        let mut cache = ImmutableCache::new();
        let rc = cache.add("shared".to_string());
        
        // rc is shared with cache
        assert_eq!(Rc::strong_count(&rc), 2);
    }

    #[test]
    fn test_mutable_cache() {
        let cache = MutableCache::new();
        
        cache.add("first".to_string());
        cache.add("second".to_string());
        
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(0), Some("first".to_string()));
        assert_eq!(cache.get(1), Some("second".to_string()));
    }

    #[test]
    fn test_mutable_cache_clone() {
        let cache1 = MutableCache::new();
        let cache2 = cache1.clone_handle();
        
        cache1.add("from cache1".to_string());
        cache2.add("from cache2".to_string());
        
        // Both see all entries
        assert_eq!(cache1.len(), 2);
        assert_eq!(cache2.len(), 2);
    }

    #[test]
    fn test_observable() {
        let obs = Observable::new(42);
        assert_eq!(obs.get(), 42);
        
        obs.set(100);
        assert_eq!(obs.get(), 100);
    }

    #[test]
    fn test_observable_subscribe() {
        let obs1 = Observable::new(10);
        let obs2 = obs1.subscribe();
        
        obs1.set(20);
        assert_eq!(obs2.get(), 20);
        
        obs2.set(30);
        assert_eq!(obs1.get(), 30);
    }

    #[test]
    fn test_observable_multiple_subscribers() {
        let obs1 = Observable::new(0);
        let obs2 = obs1.subscribe();
        let obs3 = obs1.subscribe();
        
        obs1.set(42);
        
        assert_eq!(obs1.get(), 42);
        assert_eq!(obs2.get(), 42);
        assert_eq!(obs3.get(), 42);
    }
}
