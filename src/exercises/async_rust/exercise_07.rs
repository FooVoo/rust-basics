//! Exercise 07: Async Closures - Using async blocks and closures
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Create async blocks
//! - Store futures in variables
//! - Work with async closures

/// Create and execute an async block.
pub async fn execute_async_block(value: i32) -> i32 {
    let future = async move {
        value * 2 + 10
    };
    
    future.await
}

/// Map values using an async operation.
pub async fn async_map(values: Vec<i32>) -> Vec<i32> {
    let mut results = vec![];
    
    for value in values {
        let result = async move {
            value * 2
        }.await;
        results.push(result);
    }
    
    results
}

/// Chain multiple async blocks.
pub async fn chain_async_blocks(x: i32) -> i32 {
    let step1 = async move {
        x + 10
    }.await;
    
    let step2 = async move {
        step1 * 2
    }.await;
    
    let step3 = async move {
        step2 - 5
    }.await;
    
    step3
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
