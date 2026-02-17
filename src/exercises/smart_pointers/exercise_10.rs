//! Exercise 10: Shared Data Structures - Use Rc for graph-like structures
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Build data structures with shared ownership
//! - Use Rc for graph nodes
//! - Handle multiple references to same data

use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub neighbors: Vec<Rc<Node>>,
}

impl Node {
    /// Create a new node without neighbors.
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            neighbors: Vec::new(),
        })
    }

    /// Create a node with neighbors.
    pub fn with_neighbors(value: i32, neighbors: Vec<Rc<Node>>) -> Rc<Self> {
        Rc::new(Node { value, neighbors })
    }

    /// Count total references to this node.
    pub fn ref_count(node: &Rc<Node>) -> usize {
        Rc::strong_count(node)
    }
}

/// Create a simple graph: A -> B, A -> C, B -> C
pub fn create_simple_graph() -> (Rc<Node>, Rc<Node>, Rc<Node>) {
    let c = Node::new(3);
    let b = Node::with_neighbors(2, vec![Rc::clone(&c)]);
    let a = Node::with_neighbors(1, vec![Rc::clone(&b), Rc::clone(&c)]);
    (a, b, c)
}

/// Sum all neighbor values.
pub fn sum_neighbor_values(node: &Rc<Node>) -> i32 {
    node.neighbors.iter().map(|n| n.value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = Node::new(42);
        assert_eq!(node.value, 42);
        assert!(node.neighbors.is_empty());
        assert_eq!(Node::ref_count(&node), 1);
    }

    #[test]
    fn test_node_with_neighbors() {
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let parent = Node::with_neighbors(0, vec![Rc::clone(&n1), Rc::clone(&n2)]);
        
        assert_eq!(parent.neighbors.len(), 2);
        assert_eq!(Node::ref_count(&n1), 2); // n1 + parent
        assert_eq!(Node::ref_count(&n2), 2); // n2 + parent
    }

    #[test]
    fn test_simple_graph() {
        let (a, b, c) = create_simple_graph();
        
        assert_eq!(a.value, 1);
        assert_eq!(b.value, 2);
        assert_eq!(c.value, 3);
        
        assert_eq!(a.neighbors.len(), 2);
        assert_eq!(b.neighbors.len(), 1);
        assert_eq!(c.neighbors.len(), 0);
        
        // C is referenced by A and B (plus original binding)
        assert_eq!(Node::ref_count(&c), 3);
        // B is referenced by A (plus original binding)
        assert_eq!(Node::ref_count(&b), 2);
    }

    #[test]
    fn test_sum_neighbor_values() {
        let (a, b, _c) = create_simple_graph();
        assert_eq!(sum_neighbor_values(&a), 5); // 2 + 3
        assert_eq!(sum_neighbor_values(&b), 3); // 3
    }

    #[test]
    fn test_shared_neighbor() {
        let shared = Node::new(100);
        let n1 = Node::with_neighbors(1, vec![Rc::clone(&shared)]);
        let n2 = Node::with_neighbors(2, vec![Rc::clone(&shared)]);
        
        // shared is referenced by n1, n2, and original binding
        assert_eq!(Node::ref_count(&shared), 3);
        assert_eq!(n1.neighbors[0].value, 100);
        assert_eq!(n2.neighbors[0].value, 100);
    }
}
