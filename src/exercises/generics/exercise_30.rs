//! Exercise 30: Generic Associated Types (GATs) and Type-Level Programming
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Master Generic Associated Types (GATs)
//! - Implement advanced type-level programming
//! - Create complex trait relationships with GATs

use std::marker::PhantomData;

/// A trait with a generic associated type.
pub trait Container {
    type Item<'a>
    where
        Self: 'a;

    fn get<'a>(&'a self) -> Self::Item<'a>;
}

/// A simple container that returns references.
pub struct SimpleContainer<T> {
    value: T,
}

impl<T> SimpleContainer<T> {
    pub fn new(value: T) -> Self {
        SimpleContainer { value }
    }
}

impl<T> Container for SimpleContainer<T> {
    type Item<'a> = &'a T where T: 'a;

    fn get<'a>(&'a self) -> Self::Item<'a> {
        &self.value
    }
}

/// A trait for collections with generic associated types.
pub trait Collection {
    type Item<'a>
    where
        Self: 'a;
    type Iter<'a>: Iterator<Item = Self::Item<'a>>
    where
        Self: 'a;

    fn iter<'a>(&'a self) -> Self::Iter<'a>;
}

/// A wrapper around Vec with GATs.
pub struct VecWrapper<T> {
    items: Vec<T>,
}

impl<T> VecWrapper<T> {
    pub fn new(items: Vec<T>) -> Self {
        VecWrapper { items }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Collection for VecWrapper<T> {
    type Item<'a> = &'a T where T: 'a;
    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a;

    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        self.items.iter()
    }
}

/// A trait for types that can be mapped with GATs.
pub trait Mappable {
    type Item<'a>
    where
        Self: 'a;

    fn map<'a, U, F>(&'a self, f: F) -> Vec<U>
    where
        F: Fn(Self::Item<'a>) -> U;
}

impl<T> Mappable for Vec<T> {
    type Item<'a> = &'a T where T: 'a;

    fn map<'a, U, F>(&'a self, f: F) -> Vec<U>
    where
        F: Fn(Self::Item<'a>) -> U,
    {
        self.iter().map(f).collect()
    }
}

/// A trait for streaming data with GATs.
pub trait Streamer {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

/// A simple counter streamer.
pub struct CounterStreamer {
    current: i32,
    max: i32,
}

impl CounterStreamer {
    pub fn new(max: i32) -> Self {
        CounterStreamer { current: 0, max }
    }
}

impl Streamer for CounterStreamer {
    type Item<'a> = i32;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.current < self.max {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

/// A trait for borrowing with GATs.
pub trait Borrowable {
    type Borrowed<'a>
    where
        Self: 'a;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a>;
}

/// A smart container that implements Borrowable.
pub struct SmartContainer<T> {
    inner: T,
    metadata: String,
}

impl<T> SmartContainer<T> {
    pub fn new(inner: T, metadata: String) -> Self {
        SmartContainer { inner, metadata }
    }
}

pub struct BorrowedContainer<'a, T> {
    inner: &'a T,
    metadata: &'a str,
}

impl<T> Borrowable for SmartContainer<T> {
    type Borrowed<'a> = BorrowedContainer<'a, T> where T: 'a;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a> {
        BorrowedContainer {
            inner: &self.inner,
            metadata: &self.metadata,
        }
    }
}

impl<'a, T> BorrowedContainer<'a, T> {
    pub fn inner(&self) -> &T {
        self.inner
    }

    pub fn metadata(&self) -> &str {
        self.metadata
    }
}

/// Type-level boolean.
pub trait Bool {
    const VALUE: bool;
}

pub struct True;
pub struct False;

impl Bool for True {
    const VALUE: bool = true;
}

impl Bool for False {
    const VALUE: bool = false;
}

/// Type-level conditional.
pub trait If {
    type Output;
}

pub struct Condition<B, T, F> {
    _phantom: PhantomData<(B, T, F)>,
}

impl<T, F> If for Condition<True, T, F> {
    type Output = T;
}

impl<T, F> If for Condition<False, T, F> {
    type Output = F;
}

/// Type-level selector using GATs.
pub trait Selector<T> {
    type Selected<'a>
    where
        Self: 'a,
        T: 'a;

    fn select<'a>(&'a self, value: &'a T) -> Self::Selected<'a>;
}

pub struct RefSelector;

impl<T> Selector<T> for RefSelector {
    type Selected<'a> = &'a T where T: 'a;

    fn select<'a>(&'a self, value: &'a T) -> Self::Selected<'a> {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_container() {
        let container = SimpleContainer::new(42);
        assert_eq!(*container.get(), 42);
    }

    #[test]
    fn test_simple_container_string() {
        let container = SimpleContainer::new("hello".to_string());
        assert_eq!(container.get(), "hello");
    }

    #[test]
    fn test_vec_wrapper() {
        let wrapper = VecWrapper::new(vec![1, 2, 3]);
        let sum: i32 = wrapper.iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_vec_wrapper_len() {
        let wrapper = VecWrapper::new(vec![1, 2, 3]);
        assert_eq!(wrapper.len(), 3);
    }

    #[test]
    fn test_mappable() {
        let vec = vec![1, 2, 3];
        let doubled = vec.map(|x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_mappable_strings() {
        let vec = vec!["hello", "world"];
        let lengths = vec.map(|s| s.len());
        assert_eq!(lengths, vec![5, 5]);
    }

    #[test]
    fn test_counter_streamer() {
        let mut streamer = CounterStreamer::new(3);
        assert_eq!(streamer.next(), Some(0));
        assert_eq!(streamer.next(), Some(1));
        assert_eq!(streamer.next(), Some(2));
        assert_eq!(streamer.next(), None);
    }

    #[test]
    fn test_smart_container() {
        let container = SmartContainer::new(42, "answer".to_string());
        let borrowed = container.borrow();
        assert_eq!(*borrowed.inner(), 42);
        assert_eq!(borrowed.metadata(), "answer");
    }

    #[test]
    fn test_smart_container_multiple_borrows() {
        let container = SmartContainer::new(vec![1, 2, 3], "numbers".to_string());
        let borrowed1 = container.borrow();
        let borrowed2 = container.borrow();
        assert_eq!(borrowed1.inner().len(), 3);
        assert_eq!(borrowed2.metadata(), "numbers");
    }

    #[test]
    fn test_type_level_bool() {
        assert_eq!(True::VALUE, true);
        assert_eq!(False::VALUE, false);
    }

    #[test]
    fn test_ref_selector() {
        let selector = RefSelector;
        let value = 42;
        let selected = selector.select(&value);
        assert_eq!(*selected, 42);
    }

    #[test]
    fn test_vec_wrapper_iteration() {
        let wrapper = VecWrapper::new(vec!["a", "b", "c"]);
        let collected: Vec<_> = wrapper.iter().copied().collect();
        assert_eq!(collected, vec!["a", "b", "c"]);
    }
}
