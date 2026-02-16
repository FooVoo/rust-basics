//! Exercise 29: Custom Future Implementation - Building futures from scratch
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement the Future trait manually
//! - Understand polling and waker mechanisms
//! - Create custom async primitives

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration, Instant};

/// A future that completes after a delay.
pub struct DelayFuture {
    when: Instant,
}

impl DelayFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            when: Instant::now() + duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            let when = self.when;
            
            tokio::spawn(async move {
                let now = Instant::now();
                if when > now {
                    sleep(when - now).await;
                }
                waker.wake();
            });
            
            Poll::Pending
        }
    }
}

/// A future that resolves with a value.
pub struct ReadyFuture<T> {
    value: Option<T>,
}

impl<T> ReadyFuture<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T> Future for ReadyFuture<T> {
    type Output = T;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        Poll::Ready(this.value.take().unwrap())
    }
}

/// A future that can be completed externally.
pub struct CompletableFuture<T> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

struct SharedState<T> {
    completed: bool,
    value: Option<T>,
    waker: Option<Waker>,
}

impl<T> CompletableFuture<T> {
    pub fn new() -> (Self, Completer<T>) {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            value: None,
            waker: None,
        }));
        
        let future = CompletableFuture {
            shared_state: shared_state.clone(),
        };
        
        let completer = Completer {
            shared_state,
        };
        
        (future, completer)
    }
}

impl<T> Future for CompletableFuture<T> {
    type Output = T;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();
        
        if state.completed {
            Poll::Ready(state.value.take().unwrap())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct Completer<T> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

impl<T> Completer<T> {
    pub fn complete(self, value: T) {
        let mut state = self.shared_state.lock().unwrap();
        state.completed = true;
        state.value = Some(value);
        
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delay_future() {
        let start = Instant::now();
        DelayFuture::new(Duration::from_millis(50)).await;
        let elapsed = start.elapsed();
        
        assert!(elapsed >= Duration::from_millis(50));
        assert!(elapsed < Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test_ready_future() {
        let result = ReadyFuture::new(42).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_completable_future() {
        let (future, completer) = CompletableFuture::new();
        
        tokio::spawn(async move {
            sleep(Duration::from_millis(50)).await;
            completer.complete(100);
        });
        
        let result = future.await;
        assert_eq!(result, 100);
    }

    #[tokio::test]
    async fn test_completable_immediate() {
        let (future, completer) = CompletableFuture::new();
        completer.complete(42);
        
        let result = future.await;
        assert_eq!(result, 42);
    }
}
