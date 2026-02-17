//! Exercise 26: Async Recursive Functions - Handling recursive async patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement async recursive functions with Box::pin
//! - Handle recursive async patterns
//! - Manage stack growth in async recursion

use tokio::time::{sleep, Duration};

/// Async factorial using recursion.
pub fn async_factorial(n: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = u64> + Send>>  {
    todo!("Async factorial using recursion.")
}

/// Async fibonacci using recursion.
pub fn async_fibonacci(n: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = u64> + Send>>  {
    todo!("Async fibonacci using recursion.")
}

/// Recursive tree traversal.
#[derive(Clone)]
pub struct TreeNode {
    pub value: i32,
    pub children: Vec<TreeNode>,
}

pub fn traverse_tree(node: TreeNode) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<i32>> + Send>>  {
    todo!("Recursive tree traversal.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_factorial() {
        assert_eq!(async_factorial(5).await, 120);
        assert_eq!(async_factorial(0).await, 1);
        assert_eq!(async_factorial(1).await, 1);
    }

    #[tokio::test]
    async fn test_async_fibonacci() {
        assert_eq!(async_fibonacci(0).await, 0);
        assert_eq!(async_fibonacci(1).await, 1);
        assert_eq!(async_fibonacci(5).await, 5);
        assert_eq!(async_fibonacci(10).await, 55);
    }

    #[tokio::test]
    async fn test_traverse_tree() {
        let tree = TreeNode {
            value: 1,
            children: vec![
                TreeNode {
                    value: 2,
                    children: vec![
                        TreeNode { value: 4, children: vec![] },
                        TreeNode { value: 5, children: vec![] },
                    ],
                },
                TreeNode {
                    value: 3,
                    children: vec![],
                },
            ],
        };
        
        let result = traverse_tree(tree).await;
        assert_eq!(result, vec![1, 2, 4, 5, 3]);
    }
}
