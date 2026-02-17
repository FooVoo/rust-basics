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
pub async fn concurrent_reads(value: i32, num_readers: usize) -> Vec<i32>  {
    todo!("Concurrent reads with RwLock.")
}

/// Mix reads and writes with RwLock.
pub async fn mixed_read_write(initial: i32, operations: Vec<(&str, i32)>) -> i32  {
    todo!("Mix reads and writes with RwLock.")
}

/// Shared cache with concurrent access.
pub async fn shared_cache(operations: Vec<(String, Option<i32>)>) -> Vec<Option<i32>>  {
    todo!("Shared cache with concurrent access.")
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
