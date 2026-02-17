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
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn producers
    for _ in 0..n_producers {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..items_per_producer {
                tx.send(i).unwrap();
            }
        });
    }
    drop(tx);

    // Spawn consumers
    let sum = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..n_consumers)
        .map(|_| {
            let rx = Arc::clone(&rx);
            let sum = Arc::clone(&sum);
            thread::spawn(move || {
                while let Ok(value) = rx.lock().unwrap().recv() {
                    *sum.lock().unwrap() += value;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    *sum.lock().unwrap()
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
