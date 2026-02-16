//! Exercise 22: Advanced Streams - Buffering and concurrency
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use buffer_unordered for concurrent stream processing
//! - Handle backpressure with buffering
//! - Process streams concurrently

use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

/// Process stream items concurrently with buffer_unordered.
pub async fn concurrent_stream_processing(values: Vec<i32>) -> Vec<i32> {
    stream::iter(values)
        .map(|x| async move {
            sleep(Duration::from_millis(10)).await;
            x * 2
        })
        .buffer_unordered(5)
        .collect::<Vec<_>>()
        .await
}

/// Process stream with buffered map.
pub async fn buffered_map(values: Vec<i32>, buffer_size: usize) -> Vec<i32> {
    stream::iter(values)
        .map(|x| async move {
            sleep(Duration::from_millis(5)).await;
            x + 10
        })
        .buffered(buffer_size)
        .collect()
        .await
}

/// Fold over a stream asynchronously.
pub async fn stream_fold(values: Vec<i32>) -> i32 {
    stream::iter(values)
        .fold(0, |acc, x| async move { acc + x })
        .await
}

/// Chunks stream items.
pub async fn stream_chunks(values: Vec<i32>, chunk_size: usize) -> Vec<Vec<i32>> {
    stream::iter(values)
        .chunks(chunk_size)
        .collect()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_stream_processing() {
        let mut result = concurrent_stream_processing(vec![1, 2, 3, 4, 5]).await;
        result.sort();
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_buffered_map() {
        let result = buffered_map(vec![1, 2, 3], 2).await;
        assert_eq!(result, vec![11, 12, 13]);
    }

    #[tokio::test]
    async fn test_stream_fold() {
        let result = stream_fold(vec![1, 2, 3, 4, 5]).await;
        assert_eq!(result, 15);
    }

    #[tokio::test]
    async fn test_stream_chunks() {
        let result = stream_chunks(vec![1, 2, 3, 4, 5, 6, 7], 3).await;
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }
}
