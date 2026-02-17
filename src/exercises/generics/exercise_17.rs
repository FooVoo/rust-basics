//! Exercise 17: Generic Cache - Implement a generic caching system
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create a generic cache
//! - Use HashMap with generics
//! - Implement Eq and Hash bounds

use std::collections::HashMap;
use std::hash::Hash;

/// A generic cache that stores key-value pairs.
pub struct Cache<K, V>
where
    K: Eq + Hash,
{
    store: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
{
    /// Creates a new cache with a maximum size.
    pub fn new(max_size: usize) -> Self {
        todo!("Implement new")
    }

    /// Inserts a key-value pair into the cache.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!("Implement insert")
    }

    /// Gets a reference to a value by key.
    pub fn get(&self, key: &K) -> Option<&V> {
        todo!("Implement get")
    }

    /// Checks if the cache contains a key.
    pub fn contains(&self, key: &K) -> bool {
        todo!("Implement contains")
    }

    /// Returns the number of items in the cache.
    pub fn len(&self) -> usize {
        todo!("Implement len")
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        todo!("Implement is_empty")
    }

    /// Clears all items from the cache.
    pub fn clear(&mut self) {
        todo!("Implement clear")
    }

    /// Removes a key from the cache.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        todo!("Implement remove")
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Gets a clone of a value by key.
    pub fn get_cloned(&self, key: &K) -> Option<V> {
        todo!("Implement get_cloned")
    }

    /// Gets all keys as a vector.
    pub fn keys(&self) -> Vec<K> {
        todo!("Implement keys")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache = Cache::new(10);
        cache.insert("key1", 42);
        assert_eq!(cache.get(&"key1"), Some(&42));
    }

    #[test]
    fn test_cache_contains() {
        let mut cache = Cache::new(10);
        cache.insert(1, "value");
        assert!(cache.contains(&1));
        assert!(!cache.contains(&2));
    }

    #[test]
    fn test_cache_max_size() {
        let mut cache = Cache::new(2);
        cache.insert(1, "first");
        cache.insert(2, "second");
        let result = cache.insert(3, "third");
        assert_eq!(result, None); // Cache full
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_cache_update_existing() {
        let mut cache = Cache::new(2);
        cache.insert(1, "first");
        cache.insert(2, "second");
        let old = cache.insert(1, "updated");
        assert_eq!(old, Some("first"));
        assert_eq!(cache.len(), 2); // Size unchanged
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = Cache::new(10);
        cache.insert("key", 100);
        let removed = cache.remove(&"key");
        assert_eq!(removed, Some(100));
        assert!(!cache.contains(&"key"));
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = Cache::new(10);
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_get_cloned() {
        let mut cache = Cache::new(10);
        cache.insert("key", vec![1, 2, 3]);
        let cloned = cache.get_cloned(&"key").unwrap();
        assert_eq!(cloned, vec![1, 2, 3]);
    }

    #[test]
    fn test_cache_keys() {
        let mut cache = Cache::new(10);
        cache.insert(1, "one");
        cache.insert(2, "two");
        let mut keys = cache.keys();
        keys.sort();
        assert_eq!(keys, vec![1, 2]);
    }
}
