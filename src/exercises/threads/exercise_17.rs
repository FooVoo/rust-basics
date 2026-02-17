//! Exercise 17: Mutex Deadlock Avoidance
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand deadlock scenarios
//! - Implement lock ordering
//! - Avoid circular dependencies

use std::sync::{Arc, Mutex};
use std::thread;

/// Transfer value from account1 to account2 without deadlock.
/// Multiple threads may transfer simultaneously.
/// Use consistent lock ordering to avoid deadlock.
pub fn safe_transfer(
    account1: Arc<Mutex<i32>>,
    account2: Arc<Mutex<i32>>,
    amount: i32,
) {
    // Lock in consistent order based on pointer address
    let (first, second) = if Arc::as_ptr(&account1) < Arc::as_ptr(&account2) {
        (&account1, &account2)
    } else {
        (&account2, &account1)
    };

    let mut first_lock = first.lock().unwrap();
    let mut second_lock = second.lock().unwrap();

    // Determine which is which for the actual transfer
    if Arc::ptr_eq(&account1, first) {
        *first_lock -= amount;
        *second_lock += amount;
    } else {
        *second_lock -= amount;
        *first_lock += amount;
    }
}

/// Run multiple transfers in parallel and return final balances.
pub fn parallel_transfers(initial_balance: i32, n_transfers: usize) -> (i32, i32) {
    let account1 = Arc::new(Mutex::new(initial_balance));
    let account2 = Arc::new(Mutex::new(initial_balance));

    let handles: Vec<_> = (0..n_transfers)
        .map(|i| {
            let acc1 = Arc::clone(&account1);
            let acc2 = Arc::clone(&account2);
            thread::spawn(move || {
                if i % 2 == 0 {
                    safe_transfer(acc1, acc2, 10);
                } else {
                    safe_transfer(acc2, acc1, 10);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    (*account1.lock().unwrap(), *account2.lock().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_transfers() {
        // With even number of transfers, balances should be equal
        let (bal1, bal2) = parallel_transfers(100, 10);
        assert_eq!(bal1 + bal2, 200); // Total preserved
    }

    #[test]
    fn test_parallel_transfers_many() {
        let (bal1, bal2) = parallel_transfers(1000, 100);
        assert_eq!(bal1 + bal2, 2000);
    }

    #[test]
    fn test_single_transfer() {
        let account1 = Arc::new(Mutex::new(100));
        let account2 = Arc::new(Mutex::new(50));
        safe_transfer(Arc::clone(&account1), Arc::clone(&account2), 25);
        assert_eq!(*account1.lock().unwrap(), 75);
        assert_eq!(*account2.lock().unwrap(), 75);
    }
}
