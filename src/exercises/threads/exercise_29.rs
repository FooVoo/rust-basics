//! Exercise 29: Lock-Free Stack
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement lock-free data structure
//! - Use atomic pointers
//! - Handle ABA problem considerations
//! - Master complex CAS loops

use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }

            match self.head.compare_exchange(
                head,
                new_node,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            match self.head.compare_exchange(
                head,
                next,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    let data = unsafe { ptr::read(&(*head).data) };
                    unsafe { drop(Box::from_raw(head)) };
                    return Some(data);
                }
                Err(_) => continue,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

/// Test concurrent push and pop operations on lock-free stack.
pub fn test_lock_free_stack(n_threads: usize, operations_per_thread: usize) -> usize {
    use std::sync::Arc;
    use std::thread;

    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];

    // Push threads
    for i in 0..n_threads {
        let stack = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            for j in 0..operations_per_thread {
                stack.push(i * 1000 + j);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Pop all and count
    let mut count = 0;
    while stack.pop().is_some() {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_free_stack_basic() {
        let stack = LockFreeStack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_lock_free_stack_concurrent() {
        let count = test_lock_free_stack(4, 25);
        assert_eq!(count, 100);
    }

    #[test]
    fn test_lock_free_stack_many_threads() {
        let count = test_lock_free_stack(10, 10);
        assert_eq!(count, 100);
    }
}
