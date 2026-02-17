//! Exercise 23: Async Traits - Implementing async trait methods
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Work with async trait methods using async_trait
//! - Implement traits with async methods
//! - Use dynamic dispatch with async traits

use async_trait::async_trait;

#[async_trait]
pub trait AsyncProcessor: Send + Sync {
    async fn process(&self, value: i32) -> i32;
    async fn process_batch(&self, values: Vec<i32>) -> Vec<i32> {
        todo!("Implement process_batch")
    }
}

pub struct Doubler;

#[async_trait]
impl AsyncProcessor for Doubler {
    async fn process(&self, value: i32) -> i32 {
        todo!("Implement process")
    }
}

pub struct Incrementer;

#[async_trait]
impl AsyncProcessor for Incrementer {
    async fn process(&self, value: i32) -> i32 {
        todo!("Implement process")
    }
}

/// Use async trait with dynamic dispatch.
pub async fn process_with_trait(processor: &dyn AsyncProcessor, values: Vec<i32>) -> Vec<i32> {
    todo!("Implement process_with_trait")
}

/// Chain async processors.
pub async fn chain_processors(value: i32) -> i32 {
    todo!("Implement chain_processors")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_doubler() {
        let doubler = Doubler;
        assert_eq!(doubler.process(5).await, 10);
    }

    #[tokio::test]
    async fn test_incrementer() {
        let incrementer = Incrementer;
        assert_eq!(incrementer.process(5).await, 6);
    }

    #[tokio::test]
    async fn test_process_with_trait() {
        let doubler = Doubler;
        let result = process_with_trait(&doubler, vec![1, 2, 3]).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_chain_processors() {
        assert_eq!(chain_processors(5).await, 11);
    }
}
