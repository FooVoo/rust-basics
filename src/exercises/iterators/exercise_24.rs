//! Exercise 24: Double-Ended Iterator - Bidirectional iteration
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement DoubleEndedIterator
//! - Support reverse iteration
//! - Work with bidirectional state

/// A double-ended range iterator with custom step.
pub struct StepRange {
    start: i32,
    end: i32,
    step: i32,
}

impl StepRange {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        StepRange { start, end, step }
    }
}

impl Iterator for StepRange {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.start < self.end) || (self.step < 0 && self.start > self.end) {
            let current = self.start;
            self.start += self.step;
            Some(current)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.start < self.end) || (self.step < 0 && self.start > self.end) {
            // Calculate the last value that would be yielded
            let steps = ((self.end - self.start - 1) / self.step) * self.step;
            let last = self.start + steps;
            
            if (self.step > 0 && last >= self.start) || (self.step < 0 && last <= self.start) {
                self.end = last;
                Some(last)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Palindrome checker using double-ended iteration.
pub fn is_palindrome<T: PartialEq>(items: &[T]) -> bool {
    let mut iter = items.iter();
    
    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
        if front != back {
            return false;
        }
    }
    
    true
}

/// Reverse and merge two slices using double-ended iteration.
pub fn reverse_merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().rev().chain(b.iter().rev()).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_range_forward() {
        let result: Vec<_> = StepRange::new(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
        
        let result: Vec<_> = StepRange::new(1, 8, 3).collect();
        assert_eq!(result, vec![1, 4, 7]);
    }

    #[test]
    fn test_step_range_backward() {
        let mut range = StepRange::new(0, 10, 2);
        assert_eq!(range.next_back(), Some(8));
        assert_eq!(range.next_back(), Some(6));
        
        let result: Vec<_> = StepRange::new(0, 10, 2).rev().collect();
        assert_eq!(result, vec![8, 6, 4, 2, 0]);
    }

    #[test]
    fn test_step_range_both_ends() {
        let mut range = StepRange::new(0, 10, 2);
        assert_eq!(range.next(), Some(0));
        assert_eq!(range.next_back(), Some(8));
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next_back(), Some(6));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&[1, 2, 3, 2, 1]));
        assert!(!is_palindrome(&[1, 2, 3, 4, 5]));
        assert!(is_palindrome(&[1]));
        assert!(is_palindrome::<i32>(&[]));
        assert!(is_palindrome(&["a", "b", "a"]));
        assert!(!is_palindrome(&["a", "b", "c"]));
    }

    #[test]
    fn test_reverse_merge() {
        assert_eq!(reverse_merge(&[1, 2, 3], &[4, 5, 6]), vec![3, 2, 1, 6, 5, 4]);
        assert_eq!(reverse_merge(&[], &[1, 2]), vec![2, 1]);
        assert_eq!(reverse_merge(&[1], &[]), vec![1]);
    }
}
