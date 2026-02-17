//! Exercise 11: Producer-Consumer Pattern
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement producer-consumer pattern
//! - Use channels for work distribution
//! - Coordinate multiple producers and consumers

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Spawn n_producers that each send values 0..items_per_producer.
/// Spawn n_consumers that receive and sum all values.
/// Return the total sum.
pub fn producer_consumer(n_producers: usize, n_consumers: usize, items_per_producer: usize) -> usize {
    todo!("Implement producer_consumer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_consumer() {
        // 1 producer, 1 consumer, 5 items: 0+1+2+3+4 = 10
        assert_eq!(producer_consumer(1, 1, 5), 10);
        // 2 producers, 1 consumer, 5 items each: (0+1+2+3+4) * 2 = 20
        assert_eq!(producer_consumer(2, 1, 5), 20);
    }

    #[test]
    fn test_producer_consumer_multiple() {
        // 4 producers, 2 consumers, 10 items each: (0+...+9) * 4 = 45 * 4 = 180
        assert_eq!(producer_consumer(4, 2, 10), 180);
    }
}
