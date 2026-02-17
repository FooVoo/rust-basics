//! Exercise 17: OnceCell and LazyLock - Lazy initialization
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand lazy initialization patterns
//! - Work with OnceCell for one-time initialization
//! - Use lazy statics safely

use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub max_connections: usize,
    pub timeout_seconds: u64,
}

impl Config {
    pub fn new(max_connections: usize, timeout_seconds: u64) -> Self {
        todo!("Implement new")
    }
}

/// Initialize global config (can only be called once).
pub fn init_config(config: Config) -> Result<(), Config> {
    todo!("Implement init_config")
}

/// Get global config reference.
pub fn get_config() -> Option<&'static Config> {
    todo!("Implement get_config")
}

/// Get or initialize config with default.
pub fn get_or_init_config() -> &'static Config {
    todo!("Implement get_or_init_config")
}

/// A cache that initializes on first use.
pub struct LazyCache {
    data: OnceLock<Vec<i32>>,
}

impl LazyCache {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    pub fn get_or_compute(&self) -> &Vec<i32> {
        todo!("Implement get_or_compute")
    }

    pub fn is_initialized(&self) -> bool {
        todo!("Implement is_initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new(5, 60);
        assert_eq!(config.max_connections, 5);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_get_or_init_config() {
        // Reset by using a different static in each test
        let config = get_or_init_config();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_lazy_cache() {
        let cache = LazyCache::new();
        assert!(!cache.is_initialized());
        
        let data = cache.get_or_compute();
        assert_eq!(data.len(), 100);
        assert!(cache.is_initialized());
        
        // Second call returns same data
        let data2 = cache.get_or_compute();
        assert_eq!(data.len(), data2.len());
    }

    #[test]
    fn test_lazy_cache_computation_once() {
        let cache = LazyCache::new();
        
        // Get data multiple times
        let _d1 = cache.get_or_compute();
        let _d2 = cache.get_or_compute();
        let _d3 = cache.get_or_compute();
        
        // Computation only happens once
        assert!(cache.is_initialized());
    }

    #[test]
    fn test_once_lock_basic() {
        let cell: OnceLock<i32> = OnceLock::new();
        assert!(cell.get().is_none());
        
        assert!(cell.set(42).is_ok());
        assert_eq!(cell.get(), Some(&42));
        
        // Can't set again
        assert!(cell.set(100).is_err());
        assert_eq!(cell.get(), Some(&42));
    }

    #[test]
    fn test_get_or_init() {
        let cell: OnceLock<String> = OnceLock::new();
        
        let val = cell.get_or_init(|| "initialized".to_string());
        assert_eq!(val, "initialized");
        
        // Subsequent calls return same value
        let val2 = cell.get_or_init(|| "ignored".to_string());
        assert_eq!(val2, "initialized");
    }
}
