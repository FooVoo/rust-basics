//! Exercise 19: Memory Management Patterns - Ownership strategies
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Choose appropriate smart pointer for use case
//! - Understand ownership vs sharing trade-offs
//! - Design efficient memory layouts

use std::cell::RefCell;
use std::rc::Rc;

/// Strategy 1: Unique ownership with Box
pub struct UniqueList {
    head: Option<Box<Node>>,
}

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl UniqueList {
    pub fn new() -> Self  {
        todo!("Implement new")
    }

    pub fn push_front(&mut self, value: i32)  {
        todo!("Implement push_front")
    }

    pub fn len(&self) -> usize  {
        todo!("Implement len")
    }
}

/// Strategy 2: Shared ownership with Rc
#[derive(Clone)]
pub struct SharedList {
    head: Option<Rc<SharedNode>>,
}

struct SharedNode {
    value: i32,
    next: Option<Rc<SharedNode>>,
}

impl SharedList {
    pub fn new() -> Self  {
        todo!("Implement new")
    }

    pub fn prepend(&self, value: i32) -> Self  {
        todo!("Implement prepend")
    }

    pub fn len(&self) -> usize  {
        todo!("Implement len")
    }
}

/// Strategy 3: Shared mutable with Rc+RefCell
pub struct MutableSharedList {
    head: Option<Rc<RefCell<MutableNode>>>,
}

struct MutableNode {
    value: i32,
    next: Option<Rc<RefCell<MutableNode>>>,
}

impl MutableSharedList {
    pub fn new() -> Self  {
        todo!("Implement new")
    }

    pub fn push_front(&mut self, value: i32)  {
        todo!("Implement push_front")
    }

    pub fn modify_first(&self, new_value: i32) -> bool  {
        todo!("Implement modify_first")
    }

    pub fn first_value(&self) -> Option<i32>  {
        todo!("Implement first_value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_list() {
        let mut list = UniqueList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_shared_list() {
        let list1 = SharedList::new();
        let list2 = list1.prepend(1);
        let list3 = list2.prepend(2);
        
        // All three lists share data
        assert_eq!(list1.len(), 0);
        assert_eq!(list2.len(), 1);
        assert_eq!(list3.len(), 2);
    }

    #[test]
    fn test_shared_list_clone() {
        let list1 = SharedList::new().prepend(1).prepend(2);
        let list2 = list1.clone();
        
        // Both lists share the same nodes
        assert_eq!(list1.len(), 2);
        assert_eq!(list2.len(), 2);
    }

    #[test]
    fn test_mutable_shared_list() {
        let mut list = MutableSharedList::new();
        list.push_front(10);
        list.push_front(20);
        
        assert_eq!(list.first_value(), Some(20));
        
        list.modify_first(100);
        assert_eq!(list.first_value(), Some(100));
    }

    #[test]
    fn test_mutable_shared_empty() {
        let list = MutableSharedList::new();
        assert_eq!(list.first_value(), None);
        assert!(!list.modify_first(42));
    }

    #[test]
    fn test_unique_list_ownership() {
        let mut list = UniqueList::new();
        list.push_front(1);
        // list owns the node, can't be shared
        // This is enforced by the type system
    }

    #[test]
    fn test_shared_list_memory_efficiency() {
        let base = SharedList::new().prepend(1).prepend(2).prepend(3);
        let extended = base.prepend(0);
        
        // extended shares most nodes with base
        assert_eq!(base.len(), 3);
        assert_eq!(extended.len(), 4);
    }
}
