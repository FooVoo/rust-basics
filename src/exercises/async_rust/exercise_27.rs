//! Exercise 27: Async Drop and Cleanup - Managing resources in async contexts
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Handle resource cleanup in async contexts
//! - Use RAII patterns with async
//! - Implement graceful shutdown

use tokio::sync::Mutex;
use std::sync::Arc;

pub struct AsyncResource {
    id: String,
    cleanup_log: Arc<Mutex<Vec<String>>>,
}

impl AsyncResource {
    pub fn new(id: String, cleanup_log: Arc<Mutex<Vec<String>>>) -> Self {
        Self { id, cleanup_log }
    }
    
    pub async fn cleanup(&self) {
        let mut log = self.cleanup_log.lock().await;
        log.push(format!("Cleaned up {}", self.id));
    }
}

/// Manage multiple resources with proper cleanup.
pub async fn managed_resources(num_resources: usize) -> Vec<String> {
    let cleanup_log = Arc::new(Mutex::new(Vec::new()));
    let mut resources = vec![];
    
    for i in 0..num_resources {
        let resource = AsyncResource::new(
            format!("resource_{}", i),
            Arc::clone(&cleanup_log),
        );
        resources.push(resource);
    }
    
    for resource in resources {
        resource.cleanup().await;
    }
    
    let log = cleanup_log.lock().await;
    log.clone()
}

pub struct Connection {
    id: usize,
    closed: Arc<Mutex<Vec<usize>>>,
}

impl Connection {
    pub fn new(id: usize, closed: Arc<Mutex<Vec<usize>>>) -> Self {
        Self { id, closed }
    }
    
    pub async fn close(self) {
        let mut closed_list = self.closed.lock().await;
        closed_list.push(self.id);
    }
}

/// Connection pool with graceful shutdown.
pub async fn connection_pool_shutdown(num_connections: usize) -> Vec<usize> {
    let closed_list = Arc::new(Mutex::new(Vec::new()));
    let mut connections = vec![];
    
    for i in 0..num_connections {
        connections.push(Connection::new(i, Arc::clone(&closed_list)));
    }
    
    for conn in connections {
        conn.close().await;
    }
    
    let result = closed_list.lock().await;
    let mut sorted = result.clone();
    sorted.sort();
    sorted
}

/// Guard pattern for async cleanup.
pub struct AsyncGuard {
    name: String,
    log: Arc<Mutex<Vec<String>>>,
}

impl AsyncGuard {
    pub fn new(name: String, log: Arc<Mutex<Vec<String>>>) -> Self {
        Self { name, log }
    }
    
    pub async fn release(self) {
        let mut l = self.log.lock().await;
        l.push(format!("Released {}", self.name));
    }
}

pub async fn guarded_operation() -> Vec<String> {
    let log = Arc::new(Mutex::new(Vec::new()));
    
    {
        let guard1 = AsyncGuard::new("guard1".to_string(), Arc::clone(&log));
        let guard2 = AsyncGuard::new("guard2".to_string(), Arc::clone(&log));
        
        guard1.release().await;
        guard2.release().await;
    }
    
    let result = log.lock().await;
    result.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_managed_resources() {
        let log = managed_resources(3).await;
        assert_eq!(log.len(), 3);
        assert!(log.contains(&"Cleaned up resource_0".to_string()));
        assert!(log.contains(&"Cleaned up resource_2".to_string()));
    }

    #[tokio::test]
    async fn test_connection_pool_shutdown() {
        let closed = connection_pool_shutdown(5).await;
        assert_eq!(closed, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_guarded_operation() {
        let log = guarded_operation().await;
        assert_eq!(log, vec!["Released guard1", "Released guard2"]);
    }
}
