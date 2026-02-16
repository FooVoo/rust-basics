//! Exercise 14: Parallel Map
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement parallel map operation
//! - Distribute work across threads
//! - Collect results in order

use std::thread;

/// Apply function f to each element in parallel using n_threads.
/// Return results in original order.
pub fn parallel_map<T, U, F>(data: Vec<T>, n_threads: usize, f: F) -> Vec<U>
where
    T: Send + 'static + Clone,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    if data.is_empty() || n_threads == 0 {
        return Vec::new();
    }

    let f = std::sync::Arc::new(f);
    let chunk_size = (data.len() + n_threads - 1) / n_threads;
    
    let handles: Vec<_> = data
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            let f = std::sync::Arc::clone(&f);
            thread::spawn(move || {
                chunk.into_iter()
                    .map(|(idx, item)| (idx, f(item)))
                    .collect::<Vec<_>>()
            })
        })
        .collect();

    let mut results: Vec<(usize, U)> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    
    results.sort_by_key(|(idx, _)| *idx);
    results.into_iter().map(|(_, val)| val).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_map() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_map(data, 2, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_parallel_map_strings() {
        let data = vec!["a", "b", "c"];
        let result = parallel_map(data, 2, |s| s.to_uppercase());
        assert_eq!(result, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_parallel_map_empty() {
        let data: Vec<i32> = vec![];
        let result = parallel_map(data, 2, |x| x * 2);
        assert_eq!(result, Vec::<i32>::new());
    }
}
