//! Exercise 01: Basic Thread Spawning
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand how to spawn a thread
//! - Learn about thread::spawn
//! - Understand thread closures

use std::thread;

/// Spawn a thread that returns the given number multiplied by 2.
/// Wait for the thread to finish and return the result.
pub fn spawn_and_compute(n: i32) -> i32 {
    let handle = thread::spawn(move || n * 2);
    handle.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_compute() {
        assert_eq!(spawn_and_compute(5), 10);
        assert_eq!(spawn_and_compute(0), 0);
        assert_eq!(spawn_and_compute(-3), -6);
    }

    #[test]
    fn test_multiple_spawns() {
        let results: Vec<i32> = (1..=5).map(|n| spawn_and_compute(n)).collect();
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }
}
