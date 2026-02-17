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
        Lazy {
            func: Some(func),
            value: None,
        }
    }

    /// Forces evaluation and returns a reference to the value.
    pub fn force(&mut self) -> &T {
        if self.value.is_none() {
            let func = self.func.take().unwrap();
            self.value = Some(func());
        }
        self.value.as_ref().unwrap()
    }

    /// Checks if the value has been computed.
    pub fn is_evaluated(&self) -> bool {
        self.value.is_some()
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
        Deferred {
            state: DeferredState::Pending,
        }
    }

    /// Creates a deferred with a ready value.
    pub fn ready(value: T) -> Self {
        Deferred {
            state: DeferredState::Ready(value),
        }
    }

    /// Polls the deferred for a value.
    pub fn poll(&mut self) -> Poll<&T> {
        match &self.state {
            DeferredState::Ready(value) => Poll::Ready(value),
            _ => Poll::Pending,
        }
    }

    /// Sets the value, transitioning from Pending to Ready.
    pub fn set(&mut self, value: T) -> Result<(), &'static str> {
        match self.state {
            DeferredState::Pending => {
                self.state = DeferredState::Ready(value);
                Ok(())
            }
            _ => Err("Deferred already has a value"),
        }
    }

    /// Takes the value out, consuming it.
    pub fn take(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.state, DeferredState::Consumed) {
            DeferredState::Ready(value) => Some(value),
            other => {
                self.state = other;
                None
            }
        }
    }

    /// Checks if the deferred is ready.
    pub fn is_ready(&self) -> bool {
        matches!(self.state, DeferredState::Ready(_))
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
        Chain {
            first: Some(value),
            func: Some(func),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Executes the chain and returns the result.
    pub fn run(mut self) -> U {
        let value = self.first.take().unwrap();
        let func = self.func.take().unwrap();
        func(value)
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
