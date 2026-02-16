//! Exercise 25: Iterator Performance - Optimizing iterations
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand iterator performance characteristics
//! - Use size_hint for optimization
//! - Write efficient iterator code

/// Sum using fold vs manual loop comparison function.
pub fn optimized_sum(numbers: &[i32]) -> i32 {
    numbers.iter().copied().sum()
}

/// Find using short-circuiting.
pub fn find_optimized(numbers: &[i32], target: i32) -> Option<usize> {
    numbers.iter().position(|&n| n == target)
}

/// Custom iterator with accurate size_hint.
pub struct RangeMultiples {
    current: i32,
    end: i32,
    step: i32,
}

impl RangeMultiples {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        RangeMultiples {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for RangeMultiples {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.step > 0 && self.current < self.end {
            let val = self.current;
            self.current += self.step;
            Some(val)
        } else if self.step < 0 && self.current > self.end {
            let val = self.current;
            self.current += self.step;
            Some(val)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.step == 0 {
            return (0, Some(0));
        }
        
        let remaining = if self.step > 0 {
            if self.current >= self.end {
                0
            } else {
                ((self.end - self.current + self.step - 1) / self.step) as usize
            }
        } else {
            if self.current <= self.end {
                0
            } else {
                ((self.current - self.end - self.step - 1) / (-self.step)) as usize
            }
        };
        
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RangeMultiples {}

/// Efficiently process in chunks without allocating intermediate vectors.
pub fn process_chunks_efficiently(numbers: &[i32], chunk_size: usize) -> Vec<i32> {
    numbers
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().sum())
        .collect()
}

/// Reuse allocation with extend instead of multiple collects.
pub fn merge_filtered_efficiently(a: &[i32], b: &[i32], c: &[i32]) -> Vec<i32> {
    let capacity = a.len() + b.len() + c.len();
    let mut result = Vec::with_capacity(capacity);
    
    result.extend(a.iter().filter(|&&n| n > 0));
    result.extend(b.iter().filter(|&&n| n > 0));
    result.extend(c.iter().filter(|&&n| n > 0));
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_sum() {
        assert_eq!(optimized_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(optimized_sum(&[]), 0);
        assert_eq!(optimized_sum(&[-1, 1]), 0);
    }

    #[test]
    fn test_find_optimized() {
        assert_eq!(find_optimized(&[1, 2, 3, 4, 5], 3), Some(2));
        assert_eq!(find_optimized(&[1, 2, 3], 10), None);
        assert_eq!(find_optimized(&[], 1), None);
    }

    #[test]
    fn test_range_multiples_size_hint() {
        let iter = RangeMultiples::new(0, 10, 2);
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.len(), 5);
        
        let iter = RangeMultiples::new(0, 10, 3);
        assert_eq!(iter.size_hint(), (4, Some(4)));
        
        let iter = RangeMultiples::new(10, 0, -2);
        assert_eq!(iter.size_hint(), (5, Some(5)));
    }

    #[test]
    fn test_range_multiples() {
        let result: Vec<_> = RangeMultiples::new(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
        
        let result: Vec<_> = RangeMultiples::new(10, 0, -2).collect();
        assert_eq!(result, vec![10, 8, 6, 4, 2]);
    }

    #[test]
    fn test_process_chunks_efficiently() {
        assert_eq!(
            process_chunks_efficiently(&[1, 2, 3, 4, 5, 6], 2),
            vec![3, 7, 11]
        );
        assert_eq!(process_chunks_efficiently(&[], 2), vec![]);
    }

    #[test]
    fn test_merge_filtered_efficiently() {
        let result = merge_filtered_efficiently(&[1, -2, 3], &[-1, 4], &[5, -6]);
        assert_eq!(result, vec![1, 3, 4, 5]);
        
        let result = merge_filtered_efficiently(&[], &[], &[]);
        assert_eq!(result, vec![]);
    }
}
