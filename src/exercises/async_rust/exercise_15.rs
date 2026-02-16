//! Exercise 15: Async RwLock - Read-write locks for async contexts
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::sync::RwLock
//! - Understand read vs write locks
//! - Allow multiple concurrent readers

use tokio::sync::RwLock;
use std::sync::Arc;

/// Concurrent reads with RwLock.
pub async fn concurrent_reads(value: i32, num_readers: usize) -> Vec<i32> {
    let data = Arc::new(RwLock::new(value));
    let mut handles = vec![];
    
    for _ in 0..num_readers {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let read_value = data_clone.read().await;
            *read_value
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    results
}

/// Mix reads and writes with RwLock.
pub async fn mixed_read_write(initial: i32, operations: Vec<(&str, i32)>) -> i32 {
    let data = Arc::new(RwLock::new(initial));
    
    for (op, value) in operations {
        match op {
            "read" => {
                let _read = data.read().await;
            }
            "write" => {
                let mut write = data.write().await;
                *write += value;
            }
            _ => {}
        }
    }
    
    let final_value = data.read().await;
    *final_value
}

/// Shared cache with concurrent access.
pub async fn shared_cache(operations: Vec<(String, Option<i32>)>) -> Vec<Option<i32>> {
    use std::collections::HashMap;
    
    let cache = Arc::new(RwLock::new(HashMap::new()));
    let mut results = vec![];
    
    for (key, value_opt) in operations {
        match value_opt {
            Some(value) => {
                let mut cache_write = cache.write().await;
                cache_write.insert(key, value);
                results.push(Some(value));
            }
            None => {
                let cache_read = cache.read().await;
                let result = cache_read.get(&key).copied();
                results.push(result);
            }
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_reads() {
        let results = concurrent_reads(42, 5).await;
        assert_eq!(results, vec![42, 42, 42, 42, 42]);
    }

    #[tokio::test]
    async fn test_mixed_read_write() {
        let operations = vec![
            ("write", 10),
            ("read", 0),
            ("write", 5),
            ("read", 0),
        ];
        let result = mixed_read_write(0, operations).await;
        assert_eq!(result, 15);
    }

    #[tokio::test]
    async fn test_shared_cache() {
        let operations = vec![
            ("key1".to_string(), Some(10)),
            ("key2".to_string(), Some(20)),
            ("key1".to_string(), None),
            ("key3".to_string(), None),
        ];
        let results = shared_cache(operations).await;
        assert_eq!(results, vec![Some(10), Some(20), Some(10), None]);
    }
}
