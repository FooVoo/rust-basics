//! Exercise 30: Iterator Ecosystem - Building a complete iterator library
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Design cohesive iterator APIs
//! - Create extension traits for iterators
//! - Build production-ready iterator utilities

use std::collections::HashMap;
use std::hash::Hash;

/// Extension trait for additional iterator methods.
pub trait IteratorExt: Iterator {
    /// Count frequencies of items.
    fn frequencies(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        self.fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        })
    }
    
    /// Collect into groups based on a key function.
    fn group_by_key<K, F>(self, mut key_fn: F) -> HashMap<K, Vec<Self::Item>>
    where
        Self: Sized,
        K: Eq + Hash,
        F: FnMut(&Self::Item) -> K,
    {
        self.fold(HashMap::new(), |mut map, item| {
            let key = key_fn(&item);
            map.entry(key).or_insert_with(Vec::new).push(item);
            map
        })
    }
    
    /// Take elements while accumulating doesn't exceed limit.
    fn take_while_sum(self, limit: i32) -> TakeWhileSum<Self>
    where
        Self: Sized + Iterator<Item = i32>,
    {
        TakeWhileSum {
            iter: self,
            sum: 0,
            limit,
            done: false,
        }
    }
    
    /// Create batches of fixed size.
    fn batched(self, size: usize) -> Batched<Self>
    where
        Self: Sized,
    {
        Batched {
            iter: self,
            size,
        }
    }
    
    /// Intersperse a separator between elements.
    fn intersperse(self, separator: Self::Item) -> Intersperse<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Intersperse {
            iter: self.peekable(),
            separator,
            needs_separator: false,
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

/// Iterator that takes while sum is below limit.
pub struct TakeWhileSum<I> {
    iter: I,
    sum: i32,
    limit: i32,
    done: bool,
}

impl<I> Iterator for TakeWhileSum<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        
        match self.iter.next() {
            None => None,
            Some(item) => {
                if self.sum + item <= self.limit {
                    self.sum += item;
                    Some(item)
                } else {
                    self.done = true;
                    None
                }
            }
        }
    }
}

/// Iterator that yields batches.
pub struct Batched<I>
where
    I: Iterator,
{
    iter: I,
    size: usize,
}

impl<I> Iterator for Batched<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        
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

/// Iterator that intersperses a separator.
pub struct Intersperse<I>
where
    I: Iterator,
{
    iter: std::iter::Peekable<I>,
    separator: I::Item,
    needs_separator: bool,
}

impl<I> Iterator for Intersperse<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_separator {
            self.needs_separator = false;
            if self.iter.peek().is_some() {
                return Some(self.separator.clone());
            }
        }
        
        match self.iter.next() {
            None => None,
            Some(item) => {
                if self.iter.peek().is_some() {
                    self.needs_separator = true;
                }
                Some(item)
            }
        }
    }
}

/// Utility function to create a cartesian product.
pub fn cartesian<T, U>(a: Vec<T>, b: Vec<U>) -> impl Iterator<Item = (T, U)>
where
    T: Clone,
    U: Clone,
{
    a.into_iter()
        .flat_map(move |x| b.clone().into_iter().map(move |y| (x.clone(), y)))
}

/// Utility to merge and deduplicate sorted iterators.
pub fn merge_unique<I>(iterators: Vec<I>) -> impl Iterator<Item = i32>
where
    I: Iterator<Item = i32>,
{
    let mut all: Vec<_> = iterators
        .into_iter()
        .flat_map(|iter| iter.collect::<Vec<_>>())
        .collect();
    all.sort_unstable();
    all.dedup();
    all.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequencies_extension() {
        let freqs = vec![1, 2, 2, 3, 3, 3].into_iter().frequencies();
        assert_eq!(freqs.get(&1), Some(&1));
        assert_eq!(freqs.get(&2), Some(&2));
        assert_eq!(freqs.get(&3), Some(&3));
    }

    #[test]
    fn test_group_by_key_extension() {
        let groups = vec![1, 2, 3, 4, 5, 6]
            .into_iter()
            .group_by_key(|&n| n % 2);
        
        assert_eq!(groups.get(&0).unwrap(), &vec![2, 4, 6]);
        assert_eq!(groups.get(&1).unwrap(), &vec![1, 3, 5]);
    }

    #[test]
    fn test_take_while_sum() {
        let result: Vec<_> = vec![1, 2, 3, 4, 5].into_iter().take_while_sum(6).collect();
        assert_eq!(result, vec![1, 2, 3]); // 1+2+3=6, can't add 4
        
        let result: Vec<_> = vec![10, 20].into_iter().take_while_sum(5).collect();
        assert_eq!(result, vec![]); // First element exceeds limit
    }

    #[test]
    fn test_batched() {
        let batches: Vec<_> = vec![1, 2, 3, 4, 5].into_iter().batched(2).collect();
        assert_eq!(batches, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_intersperse() {
        let result: Vec<_> = vec![1, 2, 3].into_iter().intersperse(0).collect();
        assert_eq!(result, vec![1, 0, 2, 0, 3]);
        
        let result: Vec<_> = vec![1].into_iter().intersperse(0).collect();
        assert_eq!(result, vec![1]);
        
        let result: Vec<_> = Vec::<i32>::new().into_iter().intersperse(0).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_cartesian() {
        let result: Vec<_> = cartesian(vec![1, 2], vec![3, 4]).collect();
        assert_eq!(result, vec![(1, 3), (1, 4), (2, 3), (2, 4)]);
    }

    #[test]
    fn test_merge_unique() {
        let iter1 = vec![1, 3, 5].into_iter();
        let iter2 = vec![2, 3, 4].into_iter();
        let iter3 = vec![1, 2, 6].into_iter();
        
        let result: Vec<_> = merge_unique(vec![iter1, iter2, iter3]).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_chained_extensions() {
        let result: Vec<_> = vec![1, 1, 2, 2, 3, 3]
            .into_iter()
            .batched(2)
            .map(|batch| batch.into_iter().sum::<i32>())
            .collect();
        
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_complex_pipeline() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        let result: Vec<_> = numbers
            .into_iter()
            .filter(|&n| n % 2 == 0)
            .batched(2)
            .map(|batch| batch.into_iter().product::<i32>())
            .collect();
        
        assert_eq!(result, vec![8, 48, 10]); // [2*4, 6*8, 10]
    }
}
