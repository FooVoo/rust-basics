//! Exercise 28: Custom RAII Wrappers - Building safe abstractions
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Build custom RAII wrappers
//! - Ensure safety through RAII patterns
//! - Implement resource management abstractions

use std::marker::PhantomData;

pub struct Lock<'a, T> {
    data: &'a mut T,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T> Lock<'a, T> {
    pub fn new(data: &'a mut T) -> Self  {
        todo!("Implement new")
    }
    
    pub fn access(&self) -> &T  {
        todo!("Implement access")
    }
    
    pub fn access_mut(&mut self) -> &mut T  {
        todo!("Implement access_mut")
    }
}

impl<'a, T> Drop for Lock<'a, T> {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

pub struct Transaction<'a> {
    committed: bool,
    rollback_fn: Box<dyn FnMut() + 'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(rollback_fn: impl FnMut() + 'a) -> Self  {
        todo!("Implement new")
    }
    
    pub fn commit(&mut self)  {
        todo!("Implement commit")
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

pub struct ScopeGuard<F: FnOnce()> {
    cleanup: Option<F>,
}

impl<F: FnOnce()> ScopeGuard<F> {
    pub fn new(cleanup: F) -> Self  {
        todo!("Implement new")
    }
    
    pub fn disarm(mut self)  {
        todo!("Implement disarm")
    }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

/// Create a scope guard that increments a counter on drop.
pub fn create_counter_guard(counter: &mut i32) -> ScopeGuard<impl FnOnce() + '_>  {
    todo!("Create a scope guard that increments a counter on drop.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock() {
        let mut value = 42;
        {
            let mut lock = Lock::new(&mut value);
            *lock.access_mut() = 100;
            assert_eq!(*lock.access(), 100);
        }
        assert_eq!(value, 100);
    }

    #[test]
    fn test_transaction_commit() {
        let mut rolled_back = false;
        {
            let mut tx = Transaction::new(|| rolled_back = true);
            tx.commit();
        }
        assert!(!rolled_back);
    }

    #[test]
    fn test_transaction_rollback() {
        let mut rolled_back = false;
        {
            let _tx = Transaction::new(|| rolled_back = true);
            // Not committed - should rollback
        }
        assert!(rolled_back);
    }

    #[test]
    fn test_scope_guard() {
        let mut executed = false;
        {
            let _guard = ScopeGuard::new(|| executed = true);
        }
        assert!(executed);
    }

    #[test]
    fn test_scope_guard_disarm() {
        let mut executed = false;
        {
            let guard = ScopeGuard::new(|| executed = true);
            guard.disarm();
        }
        assert!(!executed);
    }

    #[test]
    fn test_counter_guard() {
        let mut counter = 0;
        {
            let _guard = create_counter_guard(&mut counter);
            // Cannot check counter here - it's mutably borrowed
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn test_multiple_guards() {
        let mut counter1 = 0;
        let mut counter2 = 0;
        {
            let _g1 = ScopeGuard::new(|| counter1 += 1);
            {
                let _g2 = ScopeGuard::new(|| counter2 += 10);
            }
            assert_eq!(counter2, 10);
        }
        assert_eq!(counter1, 1);
    }

    #[test]
    fn test_lock_nested_access() {
        let mut data = vec![1, 2, 3];
        {
            let mut lock = Lock::new(&mut data);
            lock.access_mut().push(4);
            assert_eq!(lock.access().len(), 4);
        }
        assert_eq!(data, vec![1, 2, 3, 4]);
    }
}
