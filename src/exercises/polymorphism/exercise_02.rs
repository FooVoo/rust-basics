//! Exercise 02: Method Syntax - Understanding self vs &self
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand different forms of self
//! - Know when to use self, &self, and &mut self
//! - Understand ownership in trait methods

pub trait Processable {
    fn process(&mut self);
    fn get_value(&self) -> i32;
    fn consume(self) -> i32;
}

pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new(initial: i32) -> Self {
        Counter { value: initial }
    }
}

impl Processable for Counter {
    fn process(&mut self) {
        self.value += 1;
    }
    
    fn get_value(&self) -> i32 {
        self.value
    }
    
    fn consume(self) -> i32 {
        self.value * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_increments() {
        let mut counter = Counter::new(0);
        counter.process();
        assert_eq!(counter.get_value(), 1);
        counter.process();
        assert_eq!(counter.get_value(), 2);
    }

    #[test]
    fn test_get_value_does_not_consume() {
        let counter = Counter::new(42);
        assert_eq!(counter.get_value(), 42);
        assert_eq!(counter.get_value(), 42); // Can call multiple times
    }

    #[test]
    fn test_consume_takes_ownership() {
        let mut counter = Counter::new(10);
        counter.process();
        assert_eq!(counter.consume(), 22); // (10 + 1) * 2
        // counter is now consumed, can't use it anymore
    }

    #[test]
    fn test_mutable_reference() {
        let mut counter = Counter::new(5);
        let value_before = counter.get_value();
        counter.process();
        let value_after = counter.get_value();
        assert_eq!(value_after, value_before + 1);
    }
}
