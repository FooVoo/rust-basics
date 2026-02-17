//! Exercise 08: Channel with Timeout
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use recv_timeout on channels
//! - Handle timeouts in message passing
//! - Work with RecvTimeoutError

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Try to receive a value from a channel with timeout.
/// Returns Some(value) if received, None if timeout.
pub fn receive_with_timeout(timeout_ms: u64) -> Option<i32> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(timeout_ms + 50));
        tx.send(42).unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_with_timeout_timeout() {
        assert_eq!(receive_with_timeout(10), None);
        assert_eq!(receive_with_timeout(20), None);
    }

    #[test]
    fn test_receive_with_timeout_success() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(100).unwrap();
        });
        thread::sleep(Duration::from_millis(10));
        let result = rx.recv_timeout(Duration::from_millis(100));
        assert_eq!(result, Ok(100));
    }
}
