//! Exercise 22: Custom Iterator with State - Complex state management
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Manage complex iterator state
//! - Implement stateful iteration patterns
//! - Handle edge cases in custom iterators

/// Iterator that yields running averages.
pub struct RunningAverage {
    values: Vec<f64>,
    index: usize,
}

impl RunningAverage {
    pub fn new(values: Vec<f64>) -> Self {
        RunningAverage { values, index: 0 }
    }
}

impl Iterator for RunningAverage {
    type Item = f64;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let sum: f64 = self.values[..=self.index].iter().sum();
            let avg = sum / (self.index + 1) as f64;
            self.index += 1;
            Some(avg)
        } else {
            None
        }
    }
}

/// Iterator that yields pairs of consecutive elements.
pub struct Pairwise<I>
where
    I: Iterator,
{
    iter: I,
    prev: Option<I::Item>,
}

impl<I> Pairwise<I>
where
    I: Iterator,
{
    pub fn new(mut iter: I) -> Self {
        let prev = iter.next();
        Pairwise { iter, prev }
    }
}

impl<I> Iterator for Pairwise<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev) = self.prev.take() {
            if let Some(current) = self.iter.next() {
                self.prev = Some(current.clone());
                return Some((prev, current));
            }
        }
        None
    }
}

/// Iterator that skips every nth element.
pub struct SkipEveryNth<I> {
    iter: I,
    n: usize,
    counter: usize,
}

impl<I: Iterator> SkipEveryNth<I> {
    pub fn new(iter: I, n: usize) -> Self {
        SkipEveryNth {
            iter,
            n,
            counter: 1,
        }
    }
}

impl<I: Iterator> Iterator for SkipEveryNth<I> {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(item) => {
                    if self.n == 0 || self.counter % self.n != 0 {
                        self.counter += 1;
                        return Some(item);
                    }
                    self.counter += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_average() {
        let averages: Vec<_> = RunningAverage::new(vec![1.0, 2.0, 3.0, 4.0]).collect();
        assert_eq!(averages, vec![1.0, 1.5, 2.0, 2.5]);
        
        let averages: Vec<_> = RunningAverage::new(vec![10.0]).collect();
        assert_eq!(averages, vec![10.0]);
        
        let averages: Vec<_> = RunningAverage::new(vec![]).collect();
        assert_eq!(averages, Vec::<f64>::new());
    }

    #[test]
    fn test_pairwise() {
        let pairs: Vec<_> = Pairwise::new(vec![1, 2, 3, 4].into_iter()).collect();
        assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4)]);
        
        let pairs: Vec<_> = Pairwise::new(vec![1].into_iter()).collect();
        assert_eq!(pairs, Vec::<(i32, i32)>::new());
        
        let pairs: Vec<_> = Pairwise::new(vec![].into_iter()).collect();
        assert_eq!(pairs, Vec::<(i32, i32)>::new());
    }

    #[test]
    fn test_skip_every_nth() {
        let result: Vec<_> = SkipEveryNth::new(vec![1, 2, 3, 4, 5, 6].into_iter(), 3).collect();
        assert_eq!(result, vec![1, 2, 4, 5]); // Skip 3 and 6
        
        let result: Vec<_> = SkipEveryNth::new(vec![1, 2, 3, 4].into_iter(), 2).collect();
        assert_eq!(result, vec![1, 3]); // Skip 2 and 4
        
        let result: Vec<_> = SkipEveryNth::new(vec![1, 2, 3].into_iter(), 0).collect();
        assert_eq!(result, vec![1, 2, 3]); // Skip none
    }
}
