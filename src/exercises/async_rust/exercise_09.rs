//! Exercise 09: Select - Racing futures with tokio::select!
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::select! macro
//! - Race multiple futures
//! - Handle the first completed future

use tokio::time::{sleep, Duration};

/// Return the result of whichever future completes first.
pub async fn race_two(delay1_ms: u64, delay2_ms: u64) -> String {
    tokio::select! {
        _ = sleep(Duration::from_millis(delay1_ms)) => "First".to_string(),
        _ = sleep(Duration::from_millis(delay2_ms)) => "Second".to_string(),
    }
}

/// Race multiple computations and return the first result.
pub async fn first_to_complete(values: Vec<(i32, u64)>) -> i32 {
    if values.is_empty() {
        return 0;
    }
    
    if values.len() == 1 {
        return values[0].0;
    }
    
    tokio::select! {
        result = compute_with_delay(values[0].0, values[0].1) => result,
        result = compute_with_delay(values[1].0, values[1].1) => result,
    }
}

async fn compute_with_delay(value: i32, delay_ms: u64) -> i32 {
    sleep(Duration::from_millis(delay_ms)).await;
    value * 2
}

/// Select between a computation and a timeout.
pub async fn with_fallback(value: i32, delay_ms: u64, timeout_ms: u64) -> Result<i32, String> {
    tokio::select! {
        result = compute_with_delay(value, delay_ms) => Ok(result),
        _ = sleep(Duration::from_millis(timeout_ms)) => Err("Timeout".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_race_two() {
        let result = race_two(10, 50).await;
        assert_eq!(result, "First");
        
        let result = race_two(50, 10).await;
        assert_eq!(result, "Second");
    }

    #[tokio::test]
    async fn test_first_to_complete() {
        let result = first_to_complete(vec![(5, 10), (10, 50)]).await;
        assert_eq!(result, 10);
    }

    #[tokio::test]
    async fn test_with_fallback() {
        assert_eq!(with_fallback(5, 10, 100).await, Ok(10));
        assert_eq!(with_fallback(5, 100, 10).await, Err("Timeout".to_string()));
    }
}
