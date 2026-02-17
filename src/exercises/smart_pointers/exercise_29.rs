//! Exercise 29: Unsafe Raw Pointers - Working with raw pointers
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Understand raw pointers (*const T and *mut T)
//! - Work with unsafe code blocks
//! - Implement low-level memory operations
//! - Build unsafe abstractions safely

use std::ptr::NonNull;

/// A simple arena allocator using raw pointers.
pub struct Arena {
    buffer: Vec<u8>,
    offset: usize,
}

impl Arena {
    pub fn new(capacity: usize) -> Self  {
        todo!("A simple arena allocator using raw pointers.")
    }

    /// Allocate space for a value of type T.
    pub fn alloc<T>(&mut self, value: T) -> Option<&mut T>  {
        todo!("Allocate space for a value of type T.")
    }

    pub fn used(&self) -> usize  {
        todo!("Allocate space for a value of type T.")
    }

    pub fn capacity(&self) -> usize  {
        todo!("Implement capacity")
    }

    pub fn reset(&mut self)  {
        todo!("Implement reset")
    }
}

/// A manual linked list using raw pointers.
pub struct RawLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> RawLinkedList<T> {
    pub fn new() -> Self  {
        todo!("Implement new")
    }

    pub fn push(&mut self, value: T)  {
        todo!("Implement push")
    }

    pub fn pop(&mut self) -> Option<T>  {
        todo!("Implement pop")
    }

    pub fn len(&self) -> usize  {
        todo!("Implement len")
    }

    pub fn is_empty(&self) -> bool  {
        todo!("Implement is_empty")
    }

    pub fn peek(&self) -> Option<&T>  {
        todo!("Implement peek")
    }
}

impl<T> Drop for RawLinkedList<T> {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

/// Swap two values using raw pointers.
pub unsafe fn raw_swap<T>(a: *mut T, b: *mut T) {
    unsafe {
        let temp = std::ptr::read(a);
        std::ptr::copy(b, a, 1);
        std::ptr::write(b, temp);
    }
}

/// Create a slice from a raw pointer and length.
pub unsafe fn slice_from_raw<T>(ptr: *const T, len: usize) -> &'static [T] {
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

/// Manually clone memory using raw pointers.
pub unsafe fn clone_memory<T: Clone>(src: *const T) -> *mut T {
    unsafe {
        let value = (*src).clone();
        let boxed = Box::new(value);
        Box::into_raw(boxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_alloc() {
        let mut arena = Arena::new(1024);
        
        let x = arena.alloc(42i32);
        assert!(x.is_some());
        assert_eq!(*x.unwrap(), 42);

        let y = arena.alloc(100i32);
        assert!(y.is_some());
        assert_eq!(*y.unwrap(), 100);
    }

    #[test]
    fn test_arena_capacity() {
        let mut arena = Arena::new(10);
        
        // Try to allocate more than capacity
        let _x = arena.alloc(0u64);
        let result = arena.alloc(0u64);
        assert!(result.is_none());
    }

    #[test]
    fn test_arena_reset() {
        let mut arena = Arena::new(100);
        arena.alloc(42i32);
        let used = arena.used();
        assert!(used > 0);

        arena.reset();
        assert_eq!(arena.used(), 0);
    }

    #[test]
    fn test_raw_linked_list_push() {
        let mut list = RawLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_raw_linked_list_pop() {
        let mut list = RawLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_raw_linked_list_peek() {
        let mut list = RawLinkedList::new();
        assert!(list.peek().is_none());

        list.push(10);
        assert_eq!(list.peek(), Some(&10));

        list.push(20);
        assert_eq!(list.peek(), Some(&20));
    }

    #[test]
    fn test_raw_swap() {
        let mut a = 10;
        let mut b = 20;

        unsafe {
            raw_swap(&mut a as *mut i32, &mut b as *mut i32);
        }

        assert_eq!(a, 20);
        assert_eq!(b, 10);
    }

    #[test]
    fn test_slice_from_raw() {
        let data = vec![1, 2, 3, 4, 5];
        let slice = unsafe { slice_from_raw(data.as_ptr(), data.len()) };
        assert_eq!(slice, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_clone_memory() {
        let original = String::from("test");
        let cloned_ptr = unsafe { clone_memory(&original as *const String) };
        let cloned = unsafe { Box::from_raw(cloned_ptr) };
        
        assert_eq!(*cloned, "test");
        assert_eq!(original, "test");
    }

    #[test]
    fn test_arena_multiple_types() {
        let mut arena = Arena::new(1024);
        
        {
            let x = arena.alloc(42i32);
            assert!(x.is_some());
            assert_eq!(*x.unwrap(), 42);
        }
        
        {
            let y = arena.alloc(3.14f64);
            assert!(y.is_some());
            assert_eq!(*y.unwrap(), 3.14);
        }
        
        {
            let z = arena.alloc(true);
            assert!(z.is_some());
            assert_eq!(*z.unwrap(), true);
        }
    }

    #[test]
    fn test_raw_linked_list_with_string() {
        let mut list = RawLinkedList::new();
        list.push(String::from("first"));
        list.push(String::from("second"));

        assert_eq!(list.pop(), Some(String::from("second")));
        assert_eq!(list.pop(), Some(String::from("first")));
    }
}
