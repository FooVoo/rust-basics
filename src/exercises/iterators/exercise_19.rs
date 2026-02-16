//! Exercise 19: Complex Chains - Multi-step transformations
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Build complex iterator chains
//! - Combine multiple operations efficiently
//! - Optimize iterator pipelines

use std::collections::HashMap;

/// Process text: split words, filter long ones, count frequencies.
pub fn word_frequency(text: &str, min_length: usize) -> HashMap<String, usize> {
    text.split_whitespace()
        .filter(|word| word.len() >= min_length)
        .map(|word| word.to_lowercase())
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

/// Get squares of even numbers, sum them, but cap at limit.
pub fn capped_even_squares_sum(numbers: &[i32], limit: i32) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .scan(0, |sum, n| {
            *sum += n;
            if *sum <= limit {
                Some(n)
            } else {
                None
            }
        })
        .sum()
}

/// Process scores: filter passing, normalize to 0-1, take top n.
pub fn top_normalized_scores(scores: &[f64], passing: f64, n: usize) -> Vec<f64> {
    let max_score = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    
    if max_score <= 0.0 {
        return vec![];
    }
    
    let mut normalized: Vec<_> = scores
        .iter()
        .filter(|&&s| s >= passing)
        .map(|&s| s / max_score)
        .collect();
    
    normalized.sort_by(|a, b| b.partial_cmp(a).unwrap());
    normalized.into_iter().take(n).collect()
}

/// Extract unique digits from numbers, sort them.
pub fn unique_sorted_digits(numbers: &[i32]) -> Vec<u32> {
    let mut digits: Vec<_> = numbers
        .iter()
        .flat_map(|&n| n.abs().to_string().chars().collect::<Vec<_>>())
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    digits.sort_unstable();
    digits.dedup();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency() {
        let result = word_frequency("hello world hello rust", 3);
        assert_eq!(result.get("hello"), Some(&2));
        assert_eq!(result.get("world"), Some(&1));
        assert_eq!(result.get("rust"), Some(&1));
        assert_eq!(result.len(), 3);
        
        let result = word_frequency("a bb a bb ccc", 2);
        assert_eq!(result.get("bb"), Some(&2));
        assert_eq!(result.get("ccc"), Some(&1));
        assert!(!result.contains_key("a"));
    }

    #[test]
    fn test_capped_even_squares_sum() {
        assert_eq!(capped_even_squares_sum(&[1, 2, 3, 4, 5, 6], 30), 20); // 4 + 16
        assert_eq!(capped_even_squares_sum(&[2, 4], 100), 20); // 4 + 16
        assert_eq!(capped_even_squares_sum(&[1, 3, 5], 100), 0);
    }

    #[test]
    fn test_top_normalized_scores() {
        let scores = vec![80.0, 90.0, 70.0, 95.0, 60.0];
        let result = top_normalized_scores(&scores, 70.0, 3);
        
        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.0).abs() < 0.01); // 95/95
        assert!((result[1] - 0.947).abs() < 0.01); // 90/95
        assert!((result[2] - 0.842).abs() < 0.01); // 80/95
    }

    #[test]
    fn test_unique_sorted_digits() {
        assert_eq!(unique_sorted_digits(&[123, 321, 456]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(unique_sorted_digits(&[111, 222]), vec![1, 2]);
        assert_eq!(unique_sorted_digits(&[-123, 456]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(unique_sorted_digits(&[]), vec![]);
    }
}
