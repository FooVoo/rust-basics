//! # Iterator Exercises
//!
//! ## Learning Objectives
//! - Master iterator fundamentals and traits
//! - Use iterator adapters effectively (map, filter, fold, etc.)
//! - Create custom iterators
//! - Chain iterator operations efficiently
//! - Understand lazy evaluation and iterator performance
//! - Work with collect patterns and FromIterator
//!
//! ## Topics Covered
//! - Basic iteration (for loops, iter, into_iter, iter_mut)
//! - Iterator adapters (map, filter, filter_map, flat_map)
//! - Iterator consumers (collect, fold, reduce, sum, count)
//! - Custom iterator implementations
//! - Iterator chains and composition
//! - Advanced patterns (zip, enumerate, take, skip)
//! - Collect patterns and type inference
//! - Performance considerations
//!
//! ## Difficulty Distribution
//! - Easy: 5 exercises (01-05)
//! - Medium: 8 exercises (06-13)
//! - Hard: 5 exercises (14-18)
//! - Expert: 2 exercises (19-20)

// ============================================================================
// EASY EXERCISES (01-05): Basic iteration
// ============================================================================

/// Exercise 01: Basic Sum - Sum all numbers in a vector
/// Difficulty: Easy
///
/// Use an iterator to sum all numbers in a vector.
pub fn sum_vector(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

/// Exercise 02: Filter Even Numbers - Keep only even numbers
/// Difficulty: Easy
///
/// Filter a vector to keep only even numbers.
pub fn filter_even(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

/// Exercise 03: Map and Double - Double each number
/// Difficulty: Easy
///
/// Create a new vector with each number doubled.
pub fn double_values(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|&x| x * 2).collect()
}

/// Exercise 04: Count Items - Count elements matching condition
/// Difficulty: Easy
///
/// Count how many numbers are greater than a threshold.
pub fn count_greater_than(nums: &[i32], threshold: i32) -> usize {
    nums.iter().filter(|&&x| x > threshold).count()
}

/// Exercise 05: Find First - Find first element matching condition
/// Difficulty: Easy
///
/// Find the first number greater than threshold.
pub fn find_first_greater(nums: &[i32], threshold: i32) -> Option<i32> {
    nums.iter().find(|&&x| x > threshold).copied()
}

// ============================================================================
// MEDIUM EXERCISES (06-13): Iterator adapters and chains
// ============================================================================

/// Exercise 06: Chain Multiple Operations - Map, filter, and collect
/// Difficulty: Medium
///
/// Square all numbers, filter those greater than 10, and collect.
pub fn square_and_filter(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .map(|&x| x * x)
        .filter(|&x| x > 10)
        .collect()
}

/// Exercise 07: Flat Map - Flatten nested collections
/// Difficulty: Medium
///
/// Flatten a vector of vectors into a single vector.
pub fn flatten_vectors(nested: &[Vec<i32>]) -> Vec<i32> {
    nested.iter().flat_map(|v| v.iter()).copied().collect()
}

/// Exercise 08: Fold Operation - Compute product using fold
/// Difficulty: Medium
///
/// Calculate the product of all numbers using fold.
pub fn product(nums: &[i32]) -> i32 {
    nums.iter().fold(1, |acc, &x| acc * x)
}

/// Exercise 09: Zip Two Iterators - Combine two vectors
/// Difficulty: Medium
///
/// Create pairs from two vectors using zip.
pub fn zip_vectors(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x, y)).collect()
}

/// Exercise 10: Enumerate with Index - Transform with indices
/// Difficulty: Medium
///
/// Create strings like "0: value" for each element.
pub fn enumerate_strings(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, &s)| format!("{}: {}", i, s))
        .collect()
}

/// Exercise 11: Take and Skip - Pagination pattern
/// Difficulty: Medium
///
/// Get a page of items (skip n, take page_size).
pub fn paginate<T: Clone>(items: &[T], page: usize, page_size: usize) -> Vec<T> {
    items
        .iter()
        .skip(page * page_size)
        .take(page_size)
        .cloned()
        .collect()
}

/// Exercise 12: Partition - Split by condition
/// Difficulty: Medium
///
/// Partition numbers into even and odd.
pub fn partition_even_odd(nums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    nums.iter().copied().partition(|&x| x % 2 == 0)
}

/// Exercise 13: Filter Map - Combine filter and map
/// Difficulty: Medium
///
/// Parse strings to numbers, keeping only valid ones.
pub fn parse_valid_numbers(strings: &[&str]) -> Vec<i32> {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

// ============================================================================
// HARD EXERCISES (14-18): Custom iterators and advanced patterns
// ============================================================================

/// Exercise 14: Custom Range Iterator - Implement a step range
/// Difficulty: Hard
///
/// Create an iterator that goes from start to end with a step.
pub struct StepRange {
    current: i32,
    end: i32,
    step: i32,
}

impl StepRange {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        StepRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for StepRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current < self.end)
            || (self.step < 0 && self.current > self.end)
        {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

/// Exercise 15: Custom Fibonacci Iterator
/// Difficulty: Hard
///
/// Create an iterator that generates Fibonacci numbers.
/// The iterator will stop when overflow would occur (numbers exceed u64::MAX).
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr;
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}

/// Exercise 16: Group By Pattern - Group consecutive duplicates
/// Difficulty: Hard
///
/// Group consecutive equal elements together.
pub fn group_consecutive<T: PartialEq + Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut iter = items.iter();
    
    if let Some(first) = iter.next() {
        let mut current_group = vec![first.clone()];
        
        for item in iter {
            if item == current_group.last().unwrap() {
                current_group.push(item.clone());
            } else {
                result.push(current_group);
                current_group = vec![item.clone()];
            }
        }
        result.push(current_group);
    }
    
    result
}

/// Exercise 17: Collect into Different Types - Flexible collection
/// Difficulty: Hard
///
/// Demonstrate collecting into various collection types.
pub fn collect_to_string(nums: &[i32]) -> String {
    nums.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Exercise 18: Complex Chain - Multi-step transformation
/// Difficulty: Hard
///
/// Parse numbers, square them, filter > 100, take first 5, sum.
pub fn complex_chain(strings: &[&str]) -> i32 {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|x| x * x)
        .filter(|&x| x > 100)
        .take(5)
        .sum()
}

// ============================================================================
// EXPERT EXERCISES (19-20): Advanced iterator patterns
// ============================================================================

/// Exercise 19: Custom Iterator Adapter - Implement batching
/// Difficulty: Expert
///
/// Create an iterator that yields batches of items.
pub struct Batch<I>
where
    I: Iterator,
{
    iter: I,
    size: usize,
}

impl<I> Batch<I>
where
    I: Iterator,
{
    pub fn new(iter: I, size: usize) -> Self {
        Batch { iter, size }
    }
}

impl<I> Iterator for Batch<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut batch = Vec::with_capacity(self.size);
        
        for _ in 0..self.size {
            match self.iter.next() {
                Some(item) => batch.push(item),
                None => break,
            }
        }
        
        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
}

/// Exercise 20: Iterator with Internal State - Running average
/// Difficulty: Expert
///
/// Create an iterator that yields running averages.
pub struct RunningAverage<I>
where
    I: Iterator<Item = f64>,
{
    iter: I,
    sum: f64,
    count: usize,
}

impl<I> RunningAverage<I>
where
    I: Iterator<Item = f64>,
{
    pub fn new(iter: I) -> Self {
        RunningAverage {
            iter,
            sum: 0.0,
            count: 0,
        }
    }
}

impl<I> Iterator for RunningAverage<I>
where
    I: Iterator<Item = f64>,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(value) => {
                self.sum += value;
                self.count += 1;
                Some(self.sum / self.count as f64)
            }
            None => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Easy Tests
    #[test]
    fn test_sum_vector() {
        assert_eq!(sum_vector(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_vector(&[]), 0);
        assert_eq!(sum_vector(&[-1, 1]), 0);
    }

    #[test]
    fn test_filter_even() {
        assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(filter_even(&[1, 3, 5]), vec![]);
        assert_eq!(filter_even(&[]), vec![]);
    }

    #[test]
    fn test_double_values() {
        assert_eq!(double_values(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_values(&[]), vec![]);
        assert_eq!(double_values(&[-1, 0, 1]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_count_greater_than() {
        assert_eq!(count_greater_than(&[1, 5, 3, 8, 2], 3), 2);
        assert_eq!(count_greater_than(&[1, 2, 3], 10), 0);
        assert_eq!(count_greater_than(&[], 5), 0);
    }

    #[test]
    fn test_find_first_greater() {
        assert_eq!(find_first_greater(&[1, 5, 3, 8, 2], 3), Some(5));
        assert_eq!(find_first_greater(&[1, 2, 3], 10), None);
        assert_eq!(find_first_greater(&[], 5), None);
    }

    // Medium Tests
    #[test]
    fn test_square_and_filter() {
        assert_eq!(square_and_filter(&[1, 2, 3, 4, 5]), vec![16, 25]);
        assert_eq!(square_and_filter(&[1, 2]), vec![]);
        assert_eq!(square_and_filter(&[-4, -5]), vec![16, 25]);
    }

    #[test]
    fn test_flatten_vectors() {
        assert_eq!(
            flatten_vectors(&[vec![1, 2], vec![3, 4], vec![5]]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(flatten_vectors(&[vec![], vec![1], vec![]]), vec![1]);
        assert_eq!(flatten_vectors(&[]), vec![]);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4]), 24);
        assert_eq!(product(&[5]), 5);
        assert_eq!(product(&[2, 0, 3]), 0);
    }

    #[test]
    fn test_zip_vectors() {
        assert_eq!(
            zip_vectors(&[1, 2, 3], &[4, 5, 6]),
            vec![(1, 4), (2, 5), (3, 6)]
        );
        assert_eq!(zip_vectors(&[1, 2], &[3, 4, 5]), vec![(1, 3), (2, 4)]);
        assert_eq!(zip_vectors(&[], &[1, 2]), vec![]);
    }

    #[test]
    fn test_enumerate_strings() {
        assert_eq!(
            enumerate_strings(&["a", "b", "c"]),
            vec!["0: a", "1: b", "2: c"]
        );
        assert_eq!(enumerate_strings(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_paginate() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(paginate(&items, 0, 3), vec![1, 2, 3]);
        assert_eq!(paginate(&items, 1, 3), vec![4, 5, 6]);
        assert_eq!(paginate(&items, 3, 3), vec![10]);
        assert_eq!(paginate(&items, 10, 3), vec![]);
    }

    #[test]
    fn test_partition_even_odd() {
        let (even, odd) = partition_even_odd(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(even, vec![2, 4, 6]);
        assert_eq!(odd, vec![1, 3, 5]);
    }

    #[test]
    fn test_parse_valid_numbers() {
        assert_eq!(
            parse_valid_numbers(&["1", "abc", "2", "3.5", "4"]),
            vec![1, 2, 4]
        );
        assert_eq!(parse_valid_numbers(&["abc", "def"]), vec![]);
        assert_eq!(parse_valid_numbers(&[]), vec![]);
    }

    // Hard Tests
    #[test]
    fn test_step_range() {
        let range = StepRange::new(0, 10, 2);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_step_range_negative() {
        let range = StepRange::new(10, 0, -2);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![10, 8, 6, 4, 2]);
    }

    #[test]
    fn test_step_range_single() {
        let range = StepRange::new(5, 6, 1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_fibonacci() {
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.take(10).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_overflow() {
        let fib = Fibonacci::new();
        let count = fib.take(100).count();
        assert!(count < 100); // Should stop before overflow
    }

    #[test]
    fn test_group_consecutive() {
        assert_eq!(
            group_consecutive(&[1, 1, 2, 2, 2, 3, 1, 1]),
            vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1, 1]]
        );
        assert_eq!(group_consecutive(&[1]), vec![vec![1]]);
        assert_eq!(group_consecutive(&[1, 2, 3]), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_group_consecutive_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(group_consecutive(&empty), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_collect_to_string() {
        assert_eq!(collect_to_string(&[1, 2, 3]), "1, 2, 3");
        assert_eq!(collect_to_string(&[42]), "42");
        assert_eq!(collect_to_string(&[]), "");
    }

    #[test]
    fn test_complex_chain() {
        assert_eq!(complex_chain(&["5", "10", "12", "15", "20"]), 769);
        // 5²=25, 10²=100, 12²=144, 15²=225, 20²=400
        // Filter >100: [144, 225, 400]
        // Take 5 (all 3): sum = 769
    }

    #[test]
    fn test_complex_chain_with_invalid() {
        assert_eq!(complex_chain(&["11", "abc", "12", "def", "13"]), 434);
        // 11²=121, 12²=144, 13²=169
        // Filter >100: [121, 144, 169]
        // Sum = 121 + 144 + 169 = 434
    }

    // Expert Tests
    #[test]
    fn test_batch() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let batches: Vec<Vec<i32>> = Batch::new(nums.into_iter(), 3).collect();
        assert_eq!(batches, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn test_batch_exact() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let batches: Vec<Vec<i32>> = Batch::new(nums.into_iter(), 2).collect();
        assert_eq!(batches, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    }

    #[test]
    fn test_batch_empty() {
        let nums: Vec<i32> = vec![];
        let batches: Vec<Vec<i32>> = Batch::new(nums.into_iter(), 3).collect();
        assert_eq!(batches, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_running_average() {
        let nums = vec![1.0, 2.0, 3.0, 4.0];
        let averages: Vec<f64> = RunningAverage::new(nums.into_iter()).collect();
        assert_eq!(averages, vec![1.0, 1.5, 2.0, 2.5]);
    }

    #[test]
    fn test_running_average_single() {
        let nums = vec![5.0];
        let averages: Vec<f64> = RunningAverage::new(nums.into_iter()).collect();
        assert_eq!(averages, vec![5.0]);
    }

    #[test]
    fn test_running_average_empty() {
        let nums: Vec<f64> = vec![];
        let averages: Vec<f64> = RunningAverage::new(nums.into_iter()).collect();
        assert_eq!(averages, Vec::<f64>::new());
    }

    // Additional edge case tests
    #[test]
    fn test_filter_even_negative() {
        assert_eq!(filter_even(&[-4, -3, -2, -1, 0, 1, 2]), vec![-4, -2, 0, 2]);
    }

    #[test]
    fn test_product_with_negative() {
        assert_eq!(product(&[-1, 2, -3]), 6);
        assert_eq!(product(&[-2, -2]), 4);
    }

    #[test]
    fn test_zip_vectors_different_lengths() {
        assert_eq!(zip_vectors(&[1], &[2, 3, 4]), vec![(1, 2)]);
        assert_eq!(zip_vectors(&[1, 2, 3], &[4]), vec![(1, 4)]);
    }

    #[test]
    fn test_step_range_edge_cases() {
        let range = StepRange::new(0, 0, 1);
        assert_eq!(range.collect::<Vec<i32>>(), vec![]);
        
        let range = StepRange::new(5, 5, 1);
        assert_eq!(range.collect::<Vec<i32>>(), vec![]);
    }

    #[test]
    fn test_complex_chain_insufficient_valid() {
        // Less than 5 items after filtering
        assert_eq!(complex_chain(&["11", "12"]), 265);
        // 11²=121, 12²=144, sum=265
    }
}
