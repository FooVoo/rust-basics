//! Exercise 29: Advanced Custom Iterators - Complex iterator implementations
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement complex custom iterators
//! - Work with multiple levels of nesting
//! - Handle advanced state management

/// Iterator that flattens nested iterators with depth limit.
pub struct FlattenDepth {
    stack: Vec<Box<dyn Iterator<Item = NestedItem>>>,
    max_depth: usize,
}

#[derive(Debug, Clone)]
pub enum NestedItem {
    Value(i32),
    Nested(Vec<NestedItem>),
}

impl FlattenDepth {
    pub fn new(items: Vec<NestedItem>, max_depth: usize) -> Self {
        FlattenDepth {
            stack: vec![Box::new(items.into_iter())],
            max_depth,
        }
    }
}

impl Iterator for FlattenDepth {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut current_iter) = self.stack.pop() {
            match current_iter.next() {
                None => continue,
                Some(NestedItem::Value(v)) => {
                    self.stack.push(current_iter);
                    return Some(v);
                }
                Some(NestedItem::Nested(nested)) => {
                    if self.stack.len() < self.max_depth {
                        self.stack.push(current_iter);
                        self.stack.push(Box::new(nested.into_iter()));
                    } else {
                        self.stack.push(current_iter);
                    }
                }
            }
        }
        None
    }
}

/// Iterator that merges multiple sorted iterators.
pub struct MergeSorted<I>
where
    I: Iterator<Item = i32>,
{
    iterators: Vec<std::iter::Peekable<I>>,
}

impl<I> MergeSorted<I>
where
    I: Iterator<Item = i32>,
{
    pub fn new(iterators: Vec<I>) -> Self {
        MergeSorted {
            iterators: iterators.into_iter().map(|i| i.peekable()).collect(),
        }
    }
}

impl<I> Iterator for MergeSorted<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut min_idx = None;
        let mut min_val = None;
        
        for (idx, iter) in self.iterators.iter_mut().enumerate() {
            if let Some(&val) = iter.peek() {
                if min_val.is_none() || val < min_val.unwrap() {
                    min_val = Some(val);
                    min_idx = Some(idx);
                }
            }
        }
        
        if let Some(idx) = min_idx {
            self.iterators[idx].next()
        } else {
            None
        }
    }
}

/// Iterator that generates permutations.
pub struct Permutations<T> {
    items: Vec<T>,
    indices: Vec<usize>,
    first: bool,
}

impl<T: Clone> Permutations<T> {
    pub fn new(items: Vec<T>) -> Self {
        let indices = (0..items.len()).collect();
        Permutations {
            items,
            indices,
            first: true,
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }
        
        if self.first {
            self.first = false;
            return Some(self.indices.iter().map(|&i| self.items[i].clone()).collect());
        }
        
        // Find rightmost index i where indices[i] < indices[i+1]
        let mut i = self.indices.len().wrapping_sub(2);
        while i < self.indices.len() && self.indices[i] >= self.indices[i + 1] {
            i = i.wrapping_sub(1);
        }
        
        if i >= self.indices.len() {
            return None; // No more permutations
        }
        
        // Find rightmost index j where indices[i] < indices[j]
        let mut j = self.indices.len() - 1;
        while self.indices[i] >= self.indices[j] {
            j -= 1;
        }
        
        // Swap indices[i] and indices[j]
        self.indices.swap(i, j);
        
        // Reverse from i+1 to end
        self.indices[i + 1..].reverse();
        
        Some(self.indices.iter().map(|&idx| self.items[idx].clone()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_depth() {
        let items = vec![
            NestedItem::Value(1),
            NestedItem::Nested(vec![NestedItem::Value(2), NestedItem::Value(3)]),
            NestedItem::Value(4),
        ];
        
        let result: Vec<_> = FlattenDepth::new(items, 10).collect();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_flatten_depth_limited() {
        let items = vec![
            NestedItem::Value(1),
            NestedItem::Nested(vec![
                NestedItem::Value(2),
                NestedItem::Nested(vec![NestedItem::Value(3)]),
            ]),
        ];
        
        let result: Vec<_> = FlattenDepth::new(items, 2).collect();
        // With max_depth=2, it can go 2 levels deep, so it flattens to [1, 2, 3]
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted() {
        let iter1 = vec![1, 3, 5].into_iter();
        let iter2 = vec![2, 4, 6].into_iter();
        let iter3 = vec![0, 7].into_iter();
        
        let merged: Vec<_> = MergeSorted::new(vec![iter1, iter2, iter3]).collect();
        assert_eq!(merged, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_merge_sorted_empty() {
        let iter1 = vec![].into_iter();
        let iter2 = vec![1, 2].into_iter();
        
        let merged: Vec<_> = MergeSorted::new(vec![iter1, iter2]).collect();
        assert_eq!(merged, vec![1, 2]);
    }

    #[test]
    fn test_permutations() {
        let perms: Vec<_> = Permutations::new(vec![1, 2, 3]).collect();
        assert_eq!(perms.len(), 6); // 3! = 6
        
        assert!(perms.contains(&vec![1, 2, 3]));
        assert!(perms.contains(&vec![1, 3, 2]));
        assert!(perms.contains(&vec![2, 1, 3]));
        assert!(perms.contains(&vec![2, 3, 1]));
        assert!(perms.contains(&vec![3, 1, 2]));
        assert!(perms.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn test_permutations_small() {
        let perms: Vec<_> = Permutations::new(vec![1, 2]).collect();
        assert_eq!(perms.len(), 2);
        assert_eq!(perms, vec![vec![1, 2], vec![2, 1]]);
        
        let perms: Vec<_> = Permutations::new(vec![1]).collect();
        assert_eq!(perms, vec![vec![1]]);
        
        let perms: Vec<_> = Permutations::new(Vec::<i32>::new()).collect();
        assert_eq!(perms, Vec::<Vec<i32>>::new());
    }
}
