//! Exercise 11: Multiple References - Working with multiple borrows
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Manage multiple immutable references
//! - Understand borrow checker rules
//! - Work with references in collections

/// Find the maximum value in a slice and return a reference to it.
pub fn find_max<'a>(slice: &'a [i32]) -> Option<&'a i32> {
    slice.iter().max()
}

/// Return references to both min and max values.
pub fn find_min_max<'a>(slice: &'a [i32]) -> Option<(&'a i32, &'a i32)> {
    if slice.is_empty() {
        return None;
    }
    let min = slice.iter().min()?;
    let max = slice.iter().max()?;
    Some((min, max))
}

/// Filter and return references to even numbers.
pub fn filter_even<'a>(numbers: &'a [i32]) -> Vec<&'a i32> {
    numbers.iter().filter(|&&n| n % 2 == 0).collect()
}

/// Find the first and last elements.
pub fn first_and_last<'a, T>(slice: &'a [T]) -> Option<(&'a T, &'a T)> {
    if slice.is_empty() {
        return None;
    }
    Some((&slice[0], &slice[slice.len() - 1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max() {
        let numbers = vec![1, 5, 3, 9, 2];
        assert_eq!(find_max(&numbers), Some(&9));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(find_max(&empty), None);
    }

    #[test]
    fn test_find_min_max() {
        let numbers = vec![5, 2, 8, 1, 9];
        let result = find_min_max(&numbers);
        assert_eq!(result, Some((&1, &9)));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(find_min_max(&empty), None);
    }

    #[test]
    fn test_filter_even() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let evens = filter_even(&numbers);
        assert_eq!(evens, vec![&2, &4, &6]);
    }

    #[test]
    fn test_first_and_last() {
        let numbers = vec![10, 20, 30, 40];
        assert_eq!(first_and_last(&numbers), Some((&10, &40)));
        
        let single = vec![42];
        assert_eq!(first_and_last(&single), Some((&42, &42)));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(first_and_last(&empty), None);
    }

    #[test]
    fn test_multiple_references() {
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let max = find_max(&data);
        let evens = filter_even(&data);
        let (first, last) = first_and_last(&data).unwrap();
        
        assert_eq!(max, Some(&9));
        assert_eq!(evens.len(), 3);
        assert_eq!(first, &3);
        assert_eq!(last, &6);
    }
}
