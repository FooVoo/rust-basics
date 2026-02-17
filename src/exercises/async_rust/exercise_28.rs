//! Exercise 28: Async Context Propagation - Passing context through async calls
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Pass context through async operations
//! - Implement request tracing patterns
//! - Handle scoped values in async contexts

use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug, PartialEq)]
pub struct RequestContext {
    pub request_id: String,
    pub user_id: Option<String>,
    pub trace: Vec<String>,
}

impl RequestContext {
    pub fn new(request_id: String) -> Self  {
        todo!("Implement new")
    }
    
    pub fn with_user(mut self, user_id: String) -> Self  {
        todo!("Implement with_user")
    }
    
    pub fn add_trace(&mut self, entry: String)  {
        todo!("Implement add_trace")
    }
}

/// Chain operations with context.
pub async fn process_with_context(mut ctx: RequestContext, value: i32) -> (RequestContext, i32)  {
    todo!("Chain operations with context.")
}

/// Multi-step pipeline with context propagation.
pub async fn pipeline_with_context(ctx: RequestContext, values: Vec<i32>) -> RequestContext  {
    todo!("Multi-step pipeline with context propagation.")
}

/// Parallel operations with shared context.
pub async fn parallel_with_context(ctx: RequestContext, values: Vec<i32>) -> Vec<RequestContext>  {
    todo!("Parallel operations with shared context.")
}

/// Context with Arc for shared immutable data.
#[derive(Clone)]
pub struct SharedContext {
    pub config: Arc<Config>,
    pub request_id: String,
}

#[derive(Debug)]
pub struct Config {
    pub timeout_ms: u64,
    pub max_retries: u32,
}

impl SharedContext {
    pub fn new(config: Arc<Config>, request_id: String) -> Self  {
        todo!("Implement new")
    }
}

pub async fn operation_with_shared_context(ctx: SharedContext) -> u64  {
    todo!("Implement operation_with_shared_context")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_with_context() {
        let ctx = RequestContext::new("req-123".to_string());
        let (result_ctx, value) = process_with_context(ctx, 10).await;
        
        assert_eq!(value, 20);
        assert_eq!(result_ctx.request_id, "req-123");
        assert_eq!(result_ctx.trace.len(), 2);
    }

    #[tokio::test]
    async fn test_pipeline_with_context() {
        let ctx = RequestContext::new("req-456".to_string())
            .with_user("user-1".to_string());
        
        let result_ctx = pipeline_with_context(ctx, vec![1, 2, 3]).await;
        assert_eq!(result_ctx.trace.len(), 6);
        assert_eq!(result_ctx.user_id, Some("user-1".to_string()));
    }

    #[tokio::test]
    async fn test_parallel_with_context() {
        let ctx = RequestContext::new("req-789".to_string());
        let results = parallel_with_context(ctx, vec![5, 10]).await;
        
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.request_id == "req-789"));
    }

    #[tokio::test]
    async fn test_operation_with_shared_context() {
        let config = Arc::new(Config {
            timeout_ms: 5000,
            max_retries: 3,
        });
        let ctx = SharedContext::new(config, "req-abc".to_string());
        
        let timeout = operation_with_shared_context(ctx).await;
        assert_eq!(timeout, 5000);
    }
}
