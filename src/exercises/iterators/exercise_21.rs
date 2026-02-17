//! Exercise 21: Custom Iterator - Basic implementation
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement the Iterator trait
//! - Manage iterator state
//! - Create custom iteration logic

/// An iterator that yields squares of numbers from start to end.
pub struct SquareIterator {
    current: i32,
    end: i32,
}

impl SquareIterator {
    pub fn new(start: i32, end: i32) -> Self {
        todo!("Implement new")
    }
}

impl Iterator for SquareIterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement next")
    }
}

/// An iterator that alternates between two values.
pub struct Alternating<T> {
    first: T,
    second: T,
    use_first: bool,
}

impl<T: Clone> Alternating<T> {
    pub fn new(first: T, second: T) -> Self {
        todo!("Implement new")
    }
}

impl<T: Clone> Iterator for Alternating<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement next")
    }
}

/// An iterator that counts down from n to 0.
pub struct Countdown {
    current: i32,
}

impl Countdown {
    pub fn new(start: i32) -> Self {
        todo!("Implement new")
    }
}

impl Iterator for Countdown {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement next")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_iterator() {
        let squares: Vec<_> = SquareIterator::new(1, 5).collect();
        assert_eq!(squares, vec![1, 4, 9, 16, 25]);
        
        let squares: Vec<_> = SquareIterator::new(0, 3).collect();
        assert_eq!(squares, vec![0, 1, 4, 9]);
        
        let squares: Vec<_> = SquareIterator::new(5, 4).collect();
        assert_eq!(squares, vec![]);
    }

    #[test]
    fn test_alternating() {
        let mut alt = Alternating::new(1, 2);
        assert_eq!(alt.next(), Some(1));
        assert_eq!(alt.next(), Some(2));
        assert_eq!(alt.next(), Some(1));
        assert_eq!(alt.next(), Some(2));
        
        let values: Vec<_> = Alternating::new("a", "b").take(6).collect();
        assert_eq!(values, vec!["a", "b", "a", "b", "a", "b"]);
    }

    #[test]
    fn test_countdown() {
        let values: Vec<_> = Countdown::new(5).collect();
        assert_eq!(values, vec![5, 4, 3, 2, 1, 0]);
        
        let values: Vec<_> = Countdown::new(0).collect();
        assert_eq!(values, vec![0]);
        
        let values: Vec<_> = Countdown::new(-1).collect();
        assert_eq!(values, vec![]);
    }
}
