//! Exercise 14: Generic Iterator Wrapper - Create a generic iterator wrapper
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Work with generic iterators
//! - Implement Iterator trait generically
//! - Chain iterator operations

/// A generic iterator wrapper that adds enumeration.
pub struct Enumerated<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    index: usize,
}

impl<I, T> Enumerated<I, T>
where
    I: Iterator<Item = T>,
{
    /// Creates a new enumerated iterator.
    pub fn new(iter: I) -> Self {
        Enumerated { iter, index: 0 }
    }
}

impl<I, T> Iterator for Enumerated<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| {
            let current_index = self.index;
            self.index += 1;
            (current_index, item)
        })
    }
}

/// A generic iterator that takes elements while a predicate is true.
pub struct TakeWhile<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    iter: I,
    predicate: F,
    done: bool,
}

impl<I, T, F> TakeWhile<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    /// Creates a new TakeWhile iterator.
    pub fn new(iter: I, predicate: F) -> Self {
        TakeWhile {
            iter,
            predicate,
            done: false,
        }
    }
}

impl<I, T, F> Iterator for TakeWhile<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        
        match self.iter.next() {
            Some(item) if (self.predicate)(&item) => Some(item),
            _ => {
                self.done = true;
                None
            }
        }
    }
}

/// Helper function to create an enumerated iterator.
pub fn enumerate<I, T>(iter: I) -> Enumerated<I, T>
where
    I: Iterator<Item = T>,
{
    Enumerated::new(iter)
}

/// Helper function to create a take_while iterator.
pub fn take_while<I, T, F>(iter: I, predicate: F) -> TakeWhile<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    TakeWhile::new(iter, predicate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerated() {
        let vec = vec![10, 20, 30];
        let enumerated: Vec<_> = enumerate(vec.into_iter()).collect();
        assert_eq!(enumerated, vec![(0, 10), (1, 20), (2, 30)]);
    }

    #[test]
    fn test_enumerated_empty() {
        let vec: Vec<i32> = vec![];
        let enumerated: Vec<_> = enumerate(vec.into_iter()).collect();
        assert_eq!(enumerated.len(), 0);
    }

    #[test]
    fn test_enumerated_strings() {
        let vec = vec!["a", "b", "c"];
        let enumerated: Vec<_> = enumerate(vec.into_iter()).collect();
        assert_eq!(enumerated, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    #[test]
    fn test_take_while() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<_> = take_while(vec.into_iter(), |&x| x < 4).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_all() {
        let vec = vec![1, 2, 3];
        let result: Vec<_> = take_while(vec.into_iter(), |&x| x < 10).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_none() {
        let vec = vec![5, 6, 7];
        let result: Vec<_> = take_while(vec.into_iter(), |&x| x < 5).collect();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_take_while_strings() {
        let vec = vec!["hi", "hello", "world"];
        let result: Vec<_> = take_while(vec.into_iter(), |s| s.len() < 5).collect();
        assert_eq!(result, vec!["hi"]);
    }

    #[test]
    fn test_chained_iterators() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<_> = enumerate(take_while(vec.into_iter(), |&x| x < 4)).collect();
        assert_eq!(result, vec![(0, 1), (1, 2), (2, 3)]);
    }
}
