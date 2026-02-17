//! Exercise 13: Async Iterator Pattern - Processing sequences asynchronously
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Process collections asynchronously
//! - Implement async iterator-like patterns
//! - Transform async sequences

/// Process each item asynchronously and collect results.
pub async fn async_map<F, Fut, T, R>(items: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> Fut,
    Fut: std::future::Future<Output = R>,
 {
    todo!("Process each item asynchronously and collect results.")
}

/// Filter items using an async predicate.
pub async fn async_filter<F, Fut, T>(items: Vec<T>, pred: F) -> Vec<T>
where
    F: Fn(T) -> Fut,
    Fut: std::future::Future<Output = bool>,
    T: Clone,
 {
    todo!("Filter items using an async predicate.")
}

/// Fold items using an async operation.
pub async fn async_fold<F, Fut, T, R>(items: Vec<T>, init: R, f: F) -> R
where
    F: Fn(R, T) -> Fut,
    Fut: std::future::Future<Output = R>,
 {
    todo!("Fold items using an async operation.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_map() {
        let result = async_map(vec![1, 2, 3], |x| async move { x * 2 }).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_async_filter() {
        let result = async_filter(vec![1, 2, 3, 4, 5], |x| async move { x % 2 == 0 }).await;
        assert_eq!(result, vec![2, 4]);
    }

    #[tokio::test]
    async fn test_async_fold() {
        let result = async_fold(vec![1, 2, 3, 4], 0, |acc, x| async move { acc + x }).await;
        assert_eq!(result, 10);
    }
}
