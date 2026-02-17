//! Exercise 02: Async with Delay - Simulating async operations
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use tokio::time::sleep for delays
//! - Understand async timing operations
//! - Work with Duration

use tokio::time::{sleep, Duration};

/// Wait for the specified milliseconds, then return a message.
pub async fn delayed_greeting(name: &str, delay_ms: u64) -> String  {
    todo!("Wait for the specified milliseconds, then return a message.")
}

/// Perform a computation after a delay.
pub async fn delayed_computation(value: i32, delay_ms: u64) -> i32  {
    todo!("Perform a computation after a delay.")
}

/// Return the first value after its delay, then the second.
pub async fn sequential_delays(delay1_ms: u64, delay2_ms: u64) -> (String, String)  {
    todo!("Return the first value after its delay, then the second.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delayed_greeting() {
        let result = delayed_greeting("Alice", 10).await;
        assert_eq!(result, "Hello, Alice!");
    }

    #[tokio::test]
    async fn test_delayed_computation() {
        let result = delayed_computation(5, 10).await;
        assert_eq!(result, 10);
    }

    #[tokio::test]
    async fn test_sequential_delays() {
        let (first, second) = sequential_delays(10, 10).await;
        assert_eq!(first, "Hello, First!");
        assert_eq!(second, "Hello, Second!");
    }
}
