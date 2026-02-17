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
    todo!("Return results in original order.")
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
