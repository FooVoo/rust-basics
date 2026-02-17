//! Exercise 23: Iterator Adapters - Custom adapter patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create iterator adapters
//! - Chain custom iterators
//! - Build reusable iterator utilities

/// Iterator adapter that batches elements into fixed-size groups.
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
        todo!("Implement new")
    }
}

impl<I> Iterator for Batch<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement next")
    }
}

/// Iterator adapter that keeps only unique elements (in order of first appearance).
pub struct Unique<I>
where
    I: Iterator,
{
    iter: I,
    seen: std::collections::HashSet<String>,
}

impl<I> Unique<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        todo!("Implement new")
    }
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: ToString + Clone,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement next")
    }
}

/// Create a batching iterator from any iterator.
pub fn batch<I>(iter: I, size: usize) -> Batch<I>
where
    I: Iterator,
 {
    todo!("Create a batching iterator from any iterator.")
}

/// Create a unique iterator from any iterator.
pub fn unique<I>(iter: I) -> Unique<I>
where
    I: Iterator,
 {
    todo!("Create a unique iterator from any iterator.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch() {
        let batches: Vec<_> = Batch::new(vec![1, 2, 3, 4, 5].into_iter(), 2).collect();
        assert_eq!(batches, vec![vec![1, 2], vec![3, 4], vec![5]]);
        
        let batches: Vec<_> = Batch::new(vec![1, 2, 3, 4].into_iter(), 2).collect();
        assert_eq!(batches, vec![vec![1, 2], vec![3, 4]]);
        
        let batches: Vec<_> = Batch::new(vec![1, 2, 3].into_iter(), 5).collect();
        assert_eq!(batches, vec![vec![1, 2, 3]]);
        
        let batches: Vec<Vec<i32>> = batch(vec![].into_iter(), 2).collect();
        assert_eq!(batches, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_unique() {
        let uniq: Vec<_> = Unique::new(vec![1, 2, 2, 3, 1, 4].into_iter()).collect();
        assert_eq!(uniq, vec![1, 2, 3, 4]);
        
        let uniq: Vec<_> = unique(vec!["a", "b", "a", "c"].into_iter()).collect();
        assert_eq!(uniq, vec!["a", "b", "c"]);
        
        let uniq: Vec<_> = unique(vec![1, 1, 1].into_iter()).collect();
        assert_eq!(uniq, vec![1]);
    }

    #[test]
    fn test_batch_function() {
        let batches: Vec<_> = batch((1..=10).into_iter(), 3).collect();
        assert_eq!(batches.len(), 4);
        assert_eq!(batches[0], vec![1, 2, 3]);
        assert_eq!(batches[3], vec![10]);
    }

    #[test]
    fn test_unique_function() {
        let uniq: Vec<_> = unique(vec![5, 5, 3, 3, 1].into_iter()).collect();
        assert_eq!(uniq, vec![5, 3, 1]);
    }
}
