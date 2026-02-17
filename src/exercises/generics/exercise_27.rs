//! Exercise 27: Generic Async Future - Work with generic async patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create generic async-like structures
//! - Work with Poll and future-like types
//! - Implement state machines with generics

use std::task::Poll;

/// A generic lazy value that computes on demand.
pub struct Lazy<T, F>
where
    F: FnOnce() -> T,
{
    func: Option<F>,
    value: Option<T>,
}

impl<T, F> Lazy<T, F>
where
    F: FnOnce() -> T,
{
    /// Creates a new lazy value.
    pub fn new(func: F) -> Self {
        todo!("Implement new")
    }

    /// Forces evaluation and returns a reference to the value.
    pub fn force(&mut self) -> &T {
        todo!("Implement force")
    }

    /// Checks if the value has been computed.
    pub fn is_evaluated(&self) -> bool {
        todo!("Implement is_evaluated")
    }
}

/// A generic deferred computation.
pub struct Deferred<T> {
    state: DeferredState<T>,
}

enum DeferredState<T> {
    Pending,
    Ready(T),
    Consumed,
}

impl<T> Deferred<T> {
    /// Creates a new pending deferred.
    pub fn pending() -> Self {
        todo!("Implement pending")
    }

    /// Creates a deferred with a ready value.
    pub fn ready(value: T) -> Self {
        todo!("Implement ready")
    }

    /// Polls the deferred for a value.
    pub fn poll(&mut self) -> Poll<&T> {
        todo!("Implement poll")
    }

    /// Sets the value, transitioning from Pending to Ready.
    pub fn set(&mut self, value: T) -> Result<(), &'static str> {
        todo!("Implement set")
    }

    /// Takes the value out, consuming it.
    pub fn take(&mut self) -> Option<T> {
        todo!("Implement take")
    }

    /// Checks if the deferred is ready.
    pub fn is_ready(&self) -> bool {
        todo!("Implement is_ready")
    }
}

/// A generic chain of computations.
pub struct Chain<T, U, F>
where
    F: FnOnce(T) -> U,
{
    first: Option<T>,
    func: Option<F>,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U, F> Chain<T, U, F>
where
    F: FnOnce(T) -> U,
{
    /// Creates a new chain.
    pub fn new(value: T, func: F) -> Self {
        todo!("Implement new")
    }

    /// Executes the chain and returns the result.
    pub fn run(mut self) -> U {
        todo!("Implement run")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_new() {
        let lazy = Lazy::new(|| 42);
        assert!(!lazy.is_evaluated());
    }

    #[test]
    fn test_lazy_force() {
        let mut lazy = Lazy::new(|| 42);
        assert_eq!(*lazy.force(), 42);
        assert!(lazy.is_evaluated());
    }

    #[test]
    fn test_lazy_force_once() {
        let mut counter = 0;
        let mut lazy = Lazy::new(|| {
            counter += 1;
            counter
        });
        
        let _ = lazy.force();
        let _ = lazy.force();
        // Function is called only once
        assert_eq!(*lazy.force(), 1);
    }

    #[test]
    fn test_lazy_expensive_computation() {
        let mut lazy = Lazy::new(|| {
            // Simulate expensive computation
            (1..=10).sum::<i32>()
        });
        assert_eq!(*lazy.force(), 55);
    }

    #[test]
    fn test_deferred_pending() {
        let mut deferred: Deferred<i32> = Deferred::pending();
        assert!(matches!(deferred.poll(), Poll::Pending));
        assert!(!deferred.is_ready());
    }

    #[test]
    fn test_deferred_ready() {
        let mut deferred = Deferred::ready(42);
        assert!(deferred.is_ready());
        assert!(matches!(deferred.poll(), Poll::Ready(&42)));
    }

    #[test]
    fn test_deferred_set() {
        let mut deferred: Deferred<i32> = Deferred::pending();
        assert!(deferred.set(42).is_ok());
        assert!(deferred.is_ready());
    }

    #[test]
    fn test_deferred_set_twice() {
        let mut deferred = Deferred::ready(42);
        assert!(deferred.set(100).is_err());
    }

    #[test]
    fn test_deferred_take() {
        let mut deferred = Deferred::ready(42);
        assert_eq!(deferred.take(), Some(42));
        assert_eq!(deferred.take(), None);
    }

    #[test]
    fn test_chain() {
        let chain = Chain::new(5, |x| x * 2);
        assert_eq!(chain.run(), 10);
    }

    #[test]
    fn test_chain_type_change() {
        let chain = Chain::new(42, |x| x.to_string());
        assert_eq!(chain.run(), "42");
    }

    #[test]
    fn test_chain_complex() {
        let chain = Chain::new(vec![1, 2, 3], |v| v.iter().sum::<i32>());
        assert_eq!(chain.run(), 6);
    }
}
