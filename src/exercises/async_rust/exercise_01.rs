//! Exercise 01: Basic Async Function - Simple async computation
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand async fn syntax
//! - Learn how to call async functions with .await
//! - Return values from async functions

/// Multiply two numbers asynchronously.
/// This is a simple introduction to async functions.
pub async fn async_multiply(a: i32, b: i32) -> i32 {
    todo!("Implement async_multiply")
}

/// Add two numbers asynchronously.
pub async fn async_add(a: i32, b: i32) -> i32 {
    todo!("Implement async_add")
}

/// Chain async operations: multiply then add.
pub async fn multiply_then_add(a: i32, b: i32, c: i32) -> i32 {
    todo!("Implement multiply_then_add")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_multiply() {
        assert_eq!(async_multiply(3, 4).await, 12);
        assert_eq!(async_multiply(0, 100).await, 0);
        assert_eq!(async_multiply(-5, 6).await, -30);
    }

    #[tokio::test]
    async fn test_async_add() {
        assert_eq!(async_add(10, 20).await, 30);
        assert_eq!(async_add(-5, 5).await, 0);
    }

    #[tokio::test]
    async fn test_multiply_then_add() {
        assert_eq!(multiply_then_add(2, 3, 4).await, 10);
        assert_eq!(multiply_then_add(5, 5, 10).await, 35);
    }
}
