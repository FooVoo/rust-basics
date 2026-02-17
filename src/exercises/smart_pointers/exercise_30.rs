//! Exercise 30: Advanced Memory Patterns - Complex smart pointer patterns
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Combine multiple smart pointer concepts
//! - Implement intrusive data structures
//! - Build efficient memory management systems
//! - Master advanced ownership patterns

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::HashMap;

/// An intrusive doubly-linked list using smart pointers.
pub type NodeRef<T> = Rc<RefCell<ListNode<T>>>;
pub type WeakNodeRef<T> = Weak<RefCell<ListNode<T>>>;

pub struct ListNode<T> {
    pub value: T,
    pub prev: Option<WeakNodeRef<T>>,
    pub next: Option<NodeRef<T>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<WeakNodeRef<T>>,
    len: usize,
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> NodeRef<T> {
        Rc::new(RefCell::new(ListNode {
            value,
            prev: None,
            next: None,
        }))
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = ListNode::new(value);

        if let Some(tail_weak) = &self.tail {
            if let Some(tail_node) = tail_weak.upgrade() {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&tail_node));
                tail_node.borrow_mut().next = Some(Rc::clone(&new_node));
            }
        } else {
            self.head = Some(Rc::clone(&new_node));
        }

        self.tail = Some(Rc::downgrade(&new_node));
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = ListNode::new(value);

        if let Some(head) = &self.head {
            new_node.borrow_mut().next = Some(Rc::clone(head));
            head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        } else {
            self.tail = Some(Rc::downgrade(&new_node));
        }

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// A cache with smart pointer-based eviction policy.
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    access_order: Vec<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            access_order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front (most recently used)
            self.access_order.retain(|k| k != key);
            self.access_order.push(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update existing
            self.map.insert(key.clone(), value);
            self.access_order.retain(|k| k != &key);
            self.access_order.push(key);
        } else {
            // Add new entry
            if self.map.len() >= self.capacity {
                // Evict least recently used
                if let Some(lru_key) = self.access_order.first().cloned() {
                    self.map.remove(&lru_key);
                    self.access_order.remove(0);
                }
            }
            self.map.insert(key.clone(), value);
            self.access_order.push(key);
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// An object pool using smart pointers.
pub struct ObjectPool<T> {
    available: RefCell<Vec<Rc<RefCell<T>>>>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> ObjectPool<T> {
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        ObjectPool {
            available: RefCell::new(Vec::new()),
            factory: Box::new(factory),
        }
    }

    pub fn acquire(&self) -> Rc<RefCell<T>> {
        let mut available = self.available.borrow_mut();
        if let Some(obj) = available.pop() {
            obj
        } else {
            Rc::new(RefCell::new((self.factory)()))
        }
    }

    pub fn release(&self, obj: Rc<RefCell<T>>) {
        if Rc::strong_count(&obj) == 1 {
            self.available.borrow_mut().push(obj);
        }
    }

    pub fn available_count(&self) -> usize {
        self.available.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly_linked_list_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_doubly_linked_list_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_doubly_linked_list_mixed() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_front(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_doubly_linked_list_connections() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);

        // Verify forward links
        if let Some(head) = &list.head {
            let next = head.borrow().next.as_ref().map(|n| n.borrow().value);
            assert_eq!(next, Some(2));
        }
    }

    #[test]
    fn test_lru_cache_basic() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_cache_eviction() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");
        cache.put(3, "three"); // Evicts 1

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.get(&3), Some(&"three"));
    }

    #[test]
    fn test_lru_cache_update() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");
        cache.get(&1); // Access 1, making it most recent
        cache.put(3, "three"); // Should evict 2, not 1

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"three"));
    }

    #[test]
    fn test_lru_cache_overwrite() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(1, "ONE");

        assert_eq!(cache.get(&1), Some(&"ONE"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_object_pool_acquire() {
        let pool = ObjectPool::new(|| vec![1, 2, 3]);
        let obj1 = pool.acquire();
        let obj2 = pool.acquire();

        assert_eq!(obj1.borrow().len(), 3);
        assert_eq!(obj2.borrow().len(), 3);
    }

    #[test]
    fn test_object_pool_release() {
        let pool = ObjectPool::new(|| 0i32);
        
        let obj = pool.acquire();
        assert_eq!(pool.available_count(), 0);
        
        pool.release(obj);
        assert_eq!(pool.available_count(), 1);
    }

    #[test]
    fn test_object_pool_reuse() {
        let pool = ObjectPool::new(|| vec![0]);
        
        let obj1 = pool.acquire();
        obj1.borrow_mut().push(1);
        pool.release(obj1);

        let obj2 = pool.acquire();
        // Should reuse the same object
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn test_object_pool_multiple_refs() {
        let pool = ObjectPool::new(|| 42);
        
        let obj1 = pool.acquire();
        let _obj2 = Rc::clone(&obj1);
        
        // Can't release while multiple refs exist
        pool.release(obj1);
        assert_eq!(pool.available_count(), 0); // Not released
        
        // obj2 dropped automatically
    }

    #[test]
    fn test_empty_list() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
}
