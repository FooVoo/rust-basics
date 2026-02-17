//! Exercise 21: Stream Basics - Working with async streams
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use futures::stream
//! - Process async streams
//! - Transform and collect stream items

use futures::stream::{self, StreamExt};

/// Create a stream and collect items.
pub async fn stream_from_iter(values: Vec<i32>) -> Vec<i32> {
    stream::iter(values)
        .collect::<Vec<_>>()
        .await
}

/// Transform stream items.
pub async fn stream_map(values: Vec<i32>) -> Vec<i32> {
    stream::iter(values)
        .map(|x| x * 2)
        .collect()
        .await
}

/// Filter stream items.
pub async fn stream_filter(values: Vec<i32>) -> Vec<i32> {
    stream::iter(values)
        .filter(|x| {
            let result = *x % 2 == 0;
            async move { result }
        })
        .collect()
        .await
}

/// Chain multiple stream operations.
pub async fn stream_pipeline(values: Vec<i32>) -> Vec<i32> {
    stream::iter(values)
        .filter(|x| {
            let result = *x > 0;
            async move { result }
        })
        .map(|x| x * 2)
        .take(5)
        .collect()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stream_from_iter() {
        let result = stream_from_iter(vec![1, 2, 3, 4]).await;
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_stream_map() {
        let result = stream_map(vec![1, 2, 3]).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_stream_filter() {
        let result = stream_filter(vec![1, 2, 3, 4, 5, 6]).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_stream_pipeline() {
        let result = stream_pipeline(vec![-1, 0, 1, 2, 3, 4, 5, 6]).await;
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}
