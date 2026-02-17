//! Exercise 03: Async Result - Error handling in async functions
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Return Result from async functions
//! - Use ? operator in async context
//! - Handle errors with async/await

/// Parse a string to integer asynchronously.
pub async fn async_parse(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

/// Divide two numbers asynchronously, returning an error for division by zero.
pub async fn async_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Parse two strings and add their values.
pub async fn parse_and_add(s1: &str, s2: &str) -> Result<i32, String> {
    let n1 = async_parse(s1).await?;
    let n2 = async_parse(s2).await?;
    Ok(n1 + n2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_parse() {
        assert_eq!(async_parse("42").await, Ok(42));
        assert!(async_parse("abc").await.is_err());
    }

    #[tokio::test]
    async fn test_async_divide() {
        assert_eq!(async_divide(10, 2).await, Ok(5));
        assert_eq!(async_divide(10, 0).await, Err("Division by zero".to_string()));
    }

    #[tokio::test]
    async fn test_parse_and_add() {
        assert_eq!(parse_and_add("10", "20").await, Ok(30));
        assert!(parse_and_add("10", "abc").await.is_err());
        assert!(parse_and_add("abc", "20").await.is_err());
    }
}
