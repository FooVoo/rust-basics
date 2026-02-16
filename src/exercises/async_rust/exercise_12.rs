//! Exercise 12: Try Join - Concurrent operations with error handling
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use tokio::try_join! for fallible operations
//! - Handle errors in concurrent operations
//! - Stop on first error

use tokio::time::{sleep, Duration};

/// Execute two fallible operations concurrently.
pub async fn try_parse_both(s1: &str, s2: &str) -> Result<(i32, i32), String> {
    let parse1 = async { s1.parse::<i32>().map_err(|e| e.to_string()) };
    let parse2 = async { s2.parse::<i32>().map_err(|e| e.to_string()) };
    
    tokio::try_join!(parse1, parse2)
}

/// Concurrent division operations with error handling.
pub async fn concurrent_divisions(operations: Vec<(i32, i32)>) -> Result<Vec<i32>, String> {
    if operations.is_empty() {
        return Ok(vec![]);
    }
    
    let mut results = vec![];
    for (a, b) in operations {
        let result = divide(a, b).await?;
        results.push(result);
    }
    Ok(results)
}

async fn divide(a: i32, b: i32) -> Result<i32, String> {
    sleep(Duration::from_millis(10)).await;
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Try to parse multiple strings concurrently, fail fast on error.
pub async fn parse_all_or_none(strings: Vec<String>) -> Result<Vec<i32>, String> {
    let mut results = vec![];
    
    for s in strings {
        let parsed = async { s.parse::<i32>().map_err(|e| e.to_string()) }.await?;
        results.push(parsed);
    }
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_try_parse_both() {
        assert_eq!(try_parse_both("10", "20").await, Ok((10, 20)));
        assert!(try_parse_both("10", "abc").await.is_err());
        assert!(try_parse_both("abc", "20").await.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_divisions() {
        let result = concurrent_divisions(vec![(10, 2), (20, 4), (30, 5)]).await;
        assert_eq!(result, Ok(vec![5, 5, 6]));
        
        let result = concurrent_divisions(vec![(10, 2), (20, 0), (30, 5)]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_parse_all_or_none() {
        let result = parse_all_or_none(vec!["1".to_string(), "2".to_string(), "3".to_string()]).await;
        assert_eq!(result, Ok(vec![1, 2, 3]));
        
        let result = parse_all_or_none(vec!["1".to_string(), "abc".to_string(), "3".to_string()]).await;
        assert!(result.is_err());
    }
}
