//! Exercise 13: Rc + RefCell Pattern - Shared mutable state
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Combine Rc and RefCell for shared mutable data
//! - Understand the pattern for shared ownership with mutation
//! - Build mutable graph-like structures

use std::cell::RefCell;
use std::rc::Rc;

pub type SharedNode = Rc<RefCell<GraphNode>>;

#[derive(Debug)]
pub struct GraphNode {
    pub value: i32,
    pub neighbors: Vec<SharedNode>,
}

impl GraphNode {
    /// Create a new shared node.
    pub fn new(value: i32) -> SharedNode {
        todo!("Implement new")
    }

    /// Add a neighbor to this node.
    pub fn add_neighbor(node: &SharedNode, neighbor: SharedNode) {
        todo!("Implement add_neighbor")
    }

    /// Get the value from a shared node.
    pub fn get_value(node: &SharedNode) -> i32 {
        todo!("Implement get_value")
    }

    /// Set the value of a shared node.
    pub fn set_value(node: &SharedNode, value: i32) {
        todo!("Implement set_value")
    }

    /// Count neighbors.
    pub fn neighbor_count(node: &SharedNode) -> usize {
        todo!("Implement neighbor_count")
    }
}

/// Create a bidirectional link between two nodes.
pub fn create_bidirectional_link(a: &SharedNode, b: &SharedNode) {
    todo!("Implement create_bidirectional_link")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = GraphNode::new(42);
        assert_eq!(GraphNode::get_value(&node), 42);
        assert_eq!(GraphNode::neighbor_count(&node), 0);
    }

    #[test]
    fn test_add_neighbor() {
        let a = GraphNode::new(1);
        let b = GraphNode::new(2);
        GraphNode::add_neighbor(&a, Rc::clone(&b));
        
        assert_eq!(GraphNode::neighbor_count(&a), 1);
        assert_eq!(GraphNode::get_value(&a.borrow().neighbors[0]), 2);
    }

    #[test]
    fn test_set_value() {
        let node = GraphNode::new(10);
        GraphNode::set_value(&node, 20);
        assert_eq!(GraphNode::get_value(&node), 20);
    }

    #[test]
    fn test_bidirectional_link() {
        let a = GraphNode::new(1);
        let b = GraphNode::new(2);
        create_bidirectional_link(&a, &b);
        
        assert_eq!(GraphNode::neighbor_count(&a), 1);
        assert_eq!(GraphNode::neighbor_count(&b), 1);
        
        // Verify connections
        assert_eq!(GraphNode::get_value(&a.borrow().neighbors[0]), 2);
        assert_eq!(GraphNode::get_value(&b.borrow().neighbors[0]), 1);
    }

    #[test]
    fn test_modify_through_neighbor() {
        let a = GraphNode::new(1);
        let b = GraphNode::new(2);
        GraphNode::add_neighbor(&a, Rc::clone(&b));
        
        // Modify b through a's neighbor list
        GraphNode::set_value(&a.borrow().neighbors[0], 100);
        
        // Change should be visible through original reference
        assert_eq!(GraphNode::get_value(&b), 100);
    }

    #[test]
    fn test_shared_mutable_graph() {
        let a = GraphNode::new(1);
        let b = GraphNode::new(2);
        let c = GraphNode::new(3);
        
        GraphNode::add_neighbor(&a, Rc::clone(&b));
        GraphNode::add_neighbor(&a, Rc::clone(&c));
        GraphNode::add_neighbor(&b, Rc::clone(&c));
        
        // Create a graph: a -> b, a -> c, b -> c
        assert_eq!(GraphNode::neighbor_count(&a), 2);
        assert_eq!(GraphNode::neighbor_count(&b), 1);
        
        // c is shared by a and b
        assert_eq!(Rc::strong_count(&c), 3); // original + a + b
    }
}
