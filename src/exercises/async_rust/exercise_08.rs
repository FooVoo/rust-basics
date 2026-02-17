//! Exercise 08: Async Option - Working with Option in async context
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Return Option from async functions
//! - Handle None cases in async context
//! - Chain async Option operations

/// Find a value in a vector asynchronously.
pub async fn async_find(values: Vec<i32>, target: i32) -> Option<usize> {
    for (index, value) in values.iter().enumerate() {
        if *value == target {
            return Some(index);
        }
    }
    None
}

/// Get first valid parse result from a list of strings.
pub async fn first_valid_parse(strings: Vec<&str>) -> Option<i32> {
    for s in strings {
        if let Ok(value) = s.parse::<i32>() {
            return Some(value);
        }
    }
    None
}

/// Chain optional async operations.
pub async fn chain_optional_ops(value: Option<i32>) -> Option<i32> {
    let v1 = value?;
    let v2 = if v1 > 0 { Some(v1 * 2) } else { None }?;
    let v3 = if v2 < 100 { Some(v2 + 10) } else { None }?;
    Some(v3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_find() {
        assert_eq!(async_find(vec![1, 2, 3, 4], 3).await, Some(2));
        assert_eq!(async_find(vec![1, 2, 3, 4], 5).await, None);
    }

    #[tokio::test]
    async fn test_first_valid_parse() {
        assert_eq!(first_valid_parse(vec!["abc", "def", "42"]).await, Some(42));
        assert_eq!(first_valid_parse(vec!["abc", "def"]).await, None);
    }

    #[tokio::test]
    async fn test_chain_optional_ops() {
        assert_eq!(chain_optional_ops(Some(10)).await, Some(30));
        assert_eq!(chain_optional_ops(Some(0)).await, None);
        assert_eq!(chain_optional_ops(Some(50)).await, None);
        assert_eq!(chain_optional_ops(None).await, None);
    }
}
