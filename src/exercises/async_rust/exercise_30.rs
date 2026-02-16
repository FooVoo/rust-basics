//! Exercise 30: Advanced Async Patterns - Complex real-world scenarios
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Combine multiple async patterns
//! - Implement complex coordination scenarios
//! - Build production-ready async systems

use tokio::sync::{Semaphore, RwLock};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::collections::HashMap;

/// A rate-limited async work queue with priority.
pub struct PriorityWorkQueue {
    high_priority: Arc<RwLock<Vec<WorkItem>>>,
    low_priority: Arc<RwLock<Vec<WorkItem>>>,
    semaphore: Arc<Semaphore>,
    results: Arc<RwLock<Vec<i32>>>,
}

#[derive(Clone)]
pub struct WorkItem {
    pub id: usize,
    pub value: i32,
}

impl PriorityWorkQueue {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            high_priority: Arc::new(RwLock::new(Vec::new())),
            low_priority: Arc::new(RwLock::new(Vec::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            results: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn add_work(&self, item: WorkItem, high_priority: bool) {
        if high_priority {
            let mut queue = self.high_priority.write().await;
            queue.push(item);
        } else {
            let mut queue = self.low_priority.write().await;
            queue.push(item);
        }
    }
    
    pub async fn process_all(&self) {
        loop {
            let item = {
                let mut high = self.high_priority.write().await;
                if let Some(item) = high.pop() {
                    Some(item)
                } else {
                    let mut low = self.low_priority.write().await;
                    low.pop()
                }
            };
            
            match item {
                Some(work) => {
                    let sem = self.semaphore.clone();
                    let results = self.results.clone();
                    
                    tokio::spawn(async move {
                        let _permit = sem.acquire().await.unwrap();
                        sleep(Duration::from_millis(10)).await;
                        let result = work.value * 2;
                        
                        let mut r = results.write().await;
                        r.push(result);
                    });
                }
                None => break,
            }
        }
        
        sleep(Duration::from_millis(100)).await;
    }
    
    pub async fn get_results(&self) -> Vec<i32> {
        let results = self.results.read().await;
        let mut sorted = results.clone();
        sorted.sort();
        sorted
    }
}

/// Distributed cache with async operations.
pub struct AsyncCache {
    data: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size: usize,
}

struct CacheEntry {
    value: i32,
    expires_at: std::time::Instant,
}

impl AsyncCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<i32> {
        let cache = self.data.read().await;
        
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > std::time::Instant::now() {
                return Some(entry.value);
            }
        }
        
        None
    }
    
    pub async fn set(&self, key: String, value: i32, ttl_ms: u64) -> Result<(), String> {
        let mut cache = self.data.write().await;
        
        if cache.len() >= self.max_size && !cache.contains_key(&key) {
            return Err("Cache full".to_string());
        }
        
        cache.insert(key, CacheEntry {
            value,
            expires_at: std::time::Instant::now() + Duration::from_millis(ttl_ms),
        });
        
        Ok(())
    }
    
    pub async fn evict_expired(&self) {
        let mut cache = self.data.write().await;
        let now = std::time::Instant::now();
        
        cache.retain(|_, entry| entry.expires_at > now);
    }
}

/// Async task coordinator with dependencies.
pub struct TaskCoordinator {
    completed: Arc<RwLock<HashMap<String, i32>>>,
}

pub struct Task {
    pub id: String,
    pub dependencies: Vec<String>,
    pub work: i32,
}

impl TaskCoordinator {
    pub fn new() -> Self {
        Self {
            completed: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn execute(&self, task: Task) -> Result<i32, String> {
        for dep in &task.dependencies {
            loop {
                let completed = self.completed.read().await;
                if completed.contains_key(dep) {
                    break;
                }
                drop(completed);
                sleep(Duration::from_millis(10)).await;
            }
        }
        
        sleep(Duration::from_millis(20)).await;
        let result = task.work * 2;
        
        let mut completed = self.completed.write().await;
        completed.insert(task.id, result);
        
        Ok(result)
    }
    
    pub async fn execute_all(&self, tasks: Vec<Task>) -> HashMap<String, i32> {
        let mut handles = vec![];
        
        for task in tasks {
            let coordinator = self.clone_arc();
            let handle = tokio::spawn(async move {
                coordinator.execute(task).await
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        let completed = self.completed.read().await;
        completed.clone()
    }
    
    fn clone_arc(&self) -> Self {
        Self {
            completed: self.completed.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_priority_work_queue() {
        let queue = PriorityWorkQueue::new(2);
        
        queue.add_work(WorkItem { id: 1, value: 10 }, false).await;
        queue.add_work(WorkItem { id: 2, value: 20 }, true).await;
        queue.add_work(WorkItem { id: 3, value: 30 }, false).await;
        
        queue.process_all().await;
        
        let results = queue.get_results().await;
        assert_eq!(results, vec![20, 40, 60]);
    }

    #[tokio::test]
    async fn test_async_cache() {
        let cache = AsyncCache::new(10);
        
        cache.set("key1".to_string(), 100, 1000).await.unwrap();
        assert_eq!(cache.get("key1").await, Some(100));
        
        cache.set("key2".to_string(), 200, 10).await.unwrap();
        sleep(Duration::from_millis(20)).await;
        assert_eq!(cache.get("key2").await, None);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache = AsyncCache::new(10);
        
        cache.set("key1".to_string(), 100, 50).await.unwrap();
        sleep(Duration::from_millis(60)).await;
        
        cache.evict_expired().await;
        assert_eq!(cache.get("key1").await, None);
    }

    #[tokio::test]
    async fn test_task_coordinator() {
        let coordinator = TaskCoordinator::new();
        
        let tasks = vec![
            Task {
                id: "task1".to_string(),
                dependencies: vec![],
                work: 10,
            },
            Task {
                id: "task2".to_string(),
                dependencies: vec!["task1".to_string()],
                work: 20,
            },
            Task {
                id: "task3".to_string(),
                dependencies: vec!["task1".to_string(), "task2".to_string()],
                work: 30,
            },
        ];
        
        let results = coordinator.execute_all(tasks).await;
        
        assert_eq!(results.get("task1"), Some(&20));
        assert_eq!(results.get("task2"), Some(&40));
        assert_eq!(results.get("task3"), Some(&60));
    }
}
