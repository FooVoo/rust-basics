//! Exercise 05: Joining Futures - Concurrent execution with join
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use tokio::join! macro
//! - Execute futures concurrently
//! - Understand the difference between sequential and concurrent execution

use tokio::time::{sleep, Duration};

/// Execute two async functions concurrently and return both results.
pub async fn concurrent_multiply(a: i32, b: i32) -> (i32, i32) {
    let future1 = async { a * 2 };
    let future2 = async { b * 3 };
    
    tokio::join!(future1, future2)
}

/// Execute three delayed operations concurrently.
pub async fn concurrent_delays(delay_ms: u64) -> (String, String, String) {
    let task1 = async {
        sleep(Duration::from_millis(delay_ms)).await;
        "First".to_string()
    };
    
    let task2 = async {
        sleep(Duration::from_millis(delay_ms)).await;
        "Second".to_string()
    };
    
    let task3 = async {
        sleep(Duration::from_millis(delay_ms)).await;
        "Third".to_string()
    };
    
    tokio::join!(task1, task2, task3)
}

/// Compute sum of values concurrently.
pub async fn concurrent_sum(values: Vec<i32>) -> i32 {
    let sum1 = async { values.iter().take(values.len() / 2).sum::<i32>() };
    let sum2 = async { values.iter().skip(values.len() / 2).sum::<i32>() };
    
    let (s1, s2) = tokio::join!(sum1, sum2);
    s1 + s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_multiply() {
        assert_eq!(concurrent_multiply(5, 10).await, (10, 30));
    }

    #[tokio::test]
    async fn test_concurrent_delays() {
        let (first, second, third) = concurrent_delays(10).await;
        assert_eq!(first, "First");
        assert_eq!(second, "Second");
        assert_eq!(third, "Third");
    }

    #[tokio::test]
    async fn test_concurrent_sum() {
        assert_eq!(concurrent_sum(vec![1, 2, 3, 4, 5, 6]).await, 21);
    }
}
