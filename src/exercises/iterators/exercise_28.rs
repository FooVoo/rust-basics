//! Exercise 28: Generic Iterator Functions - Reusable utilities
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Write generic iterator functions
//! - Work with trait bounds
//! - Create reusable iterator utilities

use std::collections::HashMap;
use std::hash::Hash;

/// Generic function to find max by a key function.
pub fn max_by_key<T, K, F>(items: &[T], key_fn: F) -> Option<&T>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    items.iter().max_by_key(|item| key_fn(item))
}

/// Generic partition by predicate.
pub fn partition_by<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    items.into_iter().partition(predicate)
}

/// Generic frequency counter.
pub fn frequencies<T>(items: &[T]) -> HashMap<T, usize>
where
    T: Clone + Hash + Eq,
{
    items.iter().fold(HashMap::new(), |mut map, item| {
        *map.entry(item.clone()).or_insert(0) += 1;
        map
    })
}

/// Generic function to interleave two iterators.
pub fn interleave<I, J, T>(mut iter1: I, mut iter2: J) -> Vec<T>
where
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    let mut result = Vec::new();
    
    loop {
        match (iter1.next(), iter2.next()) {
            (Some(a), Some(b)) => {
                result.push(a);
                result.push(b);
            }
            (Some(a), None) => {
                result.push(a);
                result.extend(iter1);
                break;
            }
            (None, Some(b)) => {
                result.push(b);
                result.extend(iter2);
                break;
            }
            (None, None) => break,
        }
    }
    
    result
}

/// Generic sliding window function.
pub fn sliding_window<T, F, R>(items: &[T], size: usize, mut f: F) -> Vec<R>
where
    F: FnMut(&[T]) -> R,
{
    items.windows(size).map(|window| f(window)).collect()
}

/// Generic chunks_exact-like behavior with processing.
pub fn process_exact_chunks<T, F, R>(items: &[T], chunk_size: usize, mut f: F) -> Vec<R>
where
    F: FnMut(&[T]) -> R,
{
    items
        .chunks_exact(chunk_size)
        .map(|chunk| f(chunk))
        .collect()
}

/// Generic deduplication preserving order.
pub fn dedup_by_key<T, K, F>(items: Vec<T>, mut key_fn: F) -> Vec<T>
where
    K: Hash + Eq,
    F: FnMut(&T) -> K,
{
    let mut seen = std::collections::HashSet::new();
    items
        .into_iter()
        .filter(|item| seen.insert(key_fn(item)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_by_key() {
        let words = vec!["a", "bbb", "cc"];
        assert_eq!(max_by_key(&words, |s| s.len()), Some(&"bbb"));
        
        let numbers = vec![1, -5, 3, -2];
        assert_eq!(max_by_key(&numbers, |n: &i32| n.abs()), Some(&-5));
        
        assert_eq!(max_by_key::<i32, i32, _>(&[], |&n| n), None);
    }

    #[test]
    fn test_partition_by() {
        let (evens, odds) = partition_by(vec![1, 2, 3, 4, 5], |&n| n % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
        
        let (empty, all) = partition_by(vec![1, 2, 3], |&n| n > 10);
        assert_eq!(empty, Vec::<i32>::new());
        assert_eq!(all, vec![1, 2, 3]);
    }

    #[test]
    fn test_frequencies() {
        let freqs = frequencies(&vec![1, 2, 2, 3, 3, 3]);
        assert_eq!(freqs.get(&1), Some(&1));
        assert_eq!(freqs.get(&2), Some(&2));
        assert_eq!(freqs.get(&3), Some(&3));
        
        let freqs = frequencies(&vec!["a", "b", "a"]);
        assert_eq!(freqs.get("a"), Some(&2));
        assert_eq!(freqs.get("b"), Some(&1));
    }

    #[test]
    fn test_interleave() {
        let result = interleave(vec![1, 2, 3].into_iter(), vec![4, 5, 6].into_iter());
        assert_eq!(result, vec![1, 4, 2, 5, 3, 6]);
        
        let result = interleave(vec![1, 2].into_iter(), vec![3, 4, 5, 6].into_iter());
        assert_eq!(result, vec![1, 3, 2, 4, 5, 6]);
        
        let result = interleave(vec![1, 2, 3].into_iter(), vec![4].into_iter());
        assert_eq!(result, vec![1, 4, 2, 3]);
    }

    #[test]
    fn test_sliding_window() {
        let sums = sliding_window(&[1, 2, 3, 4, 5], 3, |w| w.iter().sum::<i32>());
        assert_eq!(sums, vec![6, 9, 12]); // [1,2,3]=6, [2,3,4]=9, [3,4,5]=12
        
        let maxes = sliding_window(&[1, 5, 3, 7, 2], 2, |w| *w.iter().max().unwrap());
        assert_eq!(maxes, vec![5, 5, 7, 7]);
    }

    #[test]
    fn test_process_exact_chunks() {
        let sums = process_exact_chunks(&[1, 2, 3, 4, 5, 6], 2, |c| c.iter().sum::<i32>());
        assert_eq!(sums, vec![3, 7, 11]);
        
        let sums = process_exact_chunks(&[1, 2, 3, 4, 5], 2, |c| c.iter().sum::<i32>());
        assert_eq!(sums, vec![3, 7]); // Last element dropped
    }

    #[test]
    fn test_dedup_by_key() {
        let result = dedup_by_key(vec![1, 2, 3, 2, 4, 1], |&n| n);
        assert_eq!(result, vec![1, 2, 3, 4]);
        
        let words = vec!["apple", "ant", "banana", "apricot"];
        let result = dedup_by_key(words, |s| s.chars().next().unwrap());
        assert_eq!(result, vec!["apple", "banana"]); // First by 'a', first by 'b'
    }
}
