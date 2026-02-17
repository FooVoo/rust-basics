//! Exercise 14: Mutable vs Immutable - Understanding borrow rules
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand mutual exclusion between mutable and immutable borrows
//! - Work within borrow checker constraints
//! - Refactor code to satisfy borrow rules

pub struct Counter {
    count: i32,
}

impl Counter {
    pub fn new() -> Self {
        todo!("Implement new")
    }
    
    pub fn increment(&mut self) {
        todo!("Implement increment")
    }
    
    pub fn get(&self) -> i32 {
        todo!("Implement get")
    }
    
    pub fn add(&mut self, amount: i32) {
        todo!("Implement add")
    }
}

/// Increment counter and return new value.
pub fn increment_and_get(counter: &mut Counter) -> i32 {
    todo!("Implement increment_and_get")
}

/// Add multiple values to counter.
pub fn add_all(counter: &mut Counter, values: &[i32]) {
    todo!("Implement add_all")
}

/// Process a vector by doubling even numbers in place.
pub fn double_evens(numbers: &mut Vec<i32>) {
    todo!("Implement double_evens")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_basic() {
        let mut counter = Counter::new();
        assert_eq!(counter.get(), 0);
        counter.increment();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_increment_and_get() {
        let mut counter = Counter::new();
        let value = increment_and_get(&mut counter);
        assert_eq!(value, 1);
        let value2 = increment_and_get(&mut counter);
        assert_eq!(value2, 2);
    }

    #[test]
    fn test_add_all() {
        let mut counter = Counter::new();
        add_all(&mut counter, &[1, 2, 3, 4]);
        assert_eq!(counter.get(), 10);
    }

    #[test]
    fn test_double_evens() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6];
        double_evens(&mut numbers);
        assert_eq!(numbers, vec![1, 4, 3, 8, 5, 12]);
    }

    #[test]
    fn test_multiple_mutations() {
        let mut counter = Counter::new();
        counter.add(5);
        counter.increment();
        add_all(&mut counter, &[2, 3]);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_borrow_rules() {
        let mut numbers = vec![2, 4, 6];
        double_evens(&mut numbers);
        // Can read after mutation
        assert_eq!(numbers.len(), 3);
        // Can mutate again
        double_evens(&mut numbers);
        assert_eq!(numbers[0], 8);
    }
}
