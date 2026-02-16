//! Exercise 26: Iterator Combinators - Advanced combinations
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Combine multiple iterator operations
//! - Use try_fold and try_for_each
//! - Handle complex iterator patterns

use std::collections::HashMap;

/// Use try_fold to find first error or compute result.
pub fn try_accumulate(numbers: &[i32]) -> Result<i32, String> {
    numbers.iter().try_fold(0, |acc, &n| {
        if n < 0 {
            Err(format!("Negative number: {}", n))
        } else {
            Ok(acc + n)
        }
    })
}

/// Process with early termination on condition.
pub fn process_until_threshold(numbers: &[i32], threshold: i32) -> Result<Vec<i32>, String> {
    let mut result = Vec::new();
    
    numbers.iter().try_for_each(|&n| {
        if n > threshold {
            Err(format!("Exceeded threshold: {}", n))
        } else {
            result.push(n * 2);
            Ok(())
        }
    })?;
    
    Ok(result)
}

/// Group elements by a key function.
pub fn group_by<T, K, F>(items: &[T], key_fn: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    items.iter().fold(HashMap::new(), |mut map, item| {
        map.entry(key_fn(item)).or_insert_with(Vec::new).push(item.clone());
        map
    })
}

/// Cartesian product of two slices.
pub fn cartesian_product(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    a.iter()
        .flat_map(|&x| b.iter().map(move |&y| (x, y)))
        .collect()
}

/// Transpose a matrix represented as Vec<Vec<T>>.
pub fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col].clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_accumulate_success() {
        assert_eq!(try_accumulate(&[1, 2, 3, 4]), Ok(10));
        assert_eq!(try_accumulate(&[]), Ok(0));
    }

    #[test]
    fn test_try_accumulate_failure() {
        assert!(try_accumulate(&[1, 2, -3, 4]).is_err());
        assert!(try_accumulate(&[-1]).is_err());
    }

    #[test]
    fn test_process_until_threshold_success() {
        assert_eq!(
            process_until_threshold(&[1, 2, 3], 10),
            Ok(vec![2, 4, 6])
        );
    }

    #[test]
    fn test_process_until_threshold_failure() {
        assert!(process_until_threshold(&[1, 20, 3], 10).is_err());
    }

    #[test]
    fn test_group_by() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let grouped = group_by(&numbers, |&n| n % 2);
        
        assert_eq!(grouped.get(&0).unwrap(), &vec![2, 4, 6]);
        assert_eq!(grouped.get(&1).unwrap(), &vec![1, 3, 5]);
    }

    #[test]
    fn test_cartesian_product() {
        assert_eq!(
            cartesian_product(&[1, 2], &[3, 4]),
            vec![(1, 3), (1, 4), (2, 3), (2, 4)]
        );
        assert_eq!(cartesian_product(&[], &[1, 2]), vec![]);
        assert_eq!(cartesian_product(&[1], &[]), vec![]);
    }

    #[test]
    fn test_transpose() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = transpose(matrix);
        assert_eq!(result, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        
        let matrix = vec![vec![1]];
        let result = transpose(matrix);
        assert_eq!(result, vec![vec![1]]);
        
        let matrix: Vec<Vec<i32>> = vec![];
        let result = transpose(matrix);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }
}
