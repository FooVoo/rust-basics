//! Exercise 07: Async Closures - Using async blocks and closures
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Create async blocks
//! - Store futures in variables
//! - Work with async closures

/// Create and execute an async block.
pub async fn execute_async_block(value: i32) -> i32 {
    todo!("Implement execute_async_block")
}

/// Map values using an async operation.
pub async fn async_map(values: Vec<i32>) -> Vec<i32> {
    todo!("Implement async_map")
}

/// Chain multiple async blocks.
pub async fn chain_async_blocks(x: i32) -> i32 {
    todo!("Implement chain_async_blocks")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_async_block() {
        assert_eq!(execute_async_block(5).await, 20);
        assert_eq!(execute_async_block(0).await, 10);
    }

    #[tokio::test]
    async fn test_async_map() {
        let result = async_map(vec![1, 2, 3, 4]).await;
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn test_chain_async_blocks() {
        assert_eq!(chain_async_blocks(5).await, 25);
        assert_eq!(chain_async_blocks(10).await, 35);
    }
}
