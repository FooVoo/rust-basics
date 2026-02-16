//! Exercise 27: Advanced Rc+RefCell - Complex shared mutable graphs
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Build complex data structures with Rc+RefCell
//! - Handle circular references safely
//! - Implement graph algorithms with shared mutable state

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeRef = Rc<RefCell<GraphNode>>;
pub type WeakNodeRef = Weak<RefCell<GraphNode>>;

#[derive(Debug)]
pub struct GraphNode {
    pub id: usize,
    pub value: i32,
    pub edges: Vec<NodeRef>,
    pub parent: Option<WeakNodeRef>,
}

impl GraphNode {
    pub fn new(id: usize, value: i32) -> NodeRef {
        Rc::new(RefCell::new(GraphNode {
            id,
            value,
            edges: Vec::new(),
            parent: None,
        }))
    }

    pub fn add_edge(from: &NodeRef, to: &NodeRef) {
        from.borrow_mut().edges.push(Rc::clone(to));
    }

    pub fn add_bidirectional(a: &NodeRef, b: &NodeRef) {
        Self::add_edge(a, b);
        Self::add_edge(b, a);
    }

    pub fn set_parent(child: &NodeRef, parent: &NodeRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
    }

    pub fn degree(node: &NodeRef) -> usize {
        node.borrow().edges.len()
    }

    pub fn has_edge_to(from: &NodeRef, to_id: usize) -> bool {
        from.borrow()
            .edges
            .iter()
            .any(|n| n.borrow().id == to_id)
    }
}

/// Build a simple graph: A <-> B <-> C
pub fn create_simple_graph() -> (NodeRef, NodeRef, NodeRef) {
    let a = GraphNode::new(1, 10);
    let b = GraphNode::new(2, 20);
    let c = GraphNode::new(3, 30);

    GraphNode::add_bidirectional(&a, &b);
    GraphNode::add_bidirectional(&b, &c);

    (a, b, c)
}

/// Calculate sum of all neighbor values.
pub fn sum_neighbor_values(node: &NodeRef) -> i32 {
    node.borrow()
        .edges
        .iter()
        .map(|n| n.borrow().value)
        .sum()
}

/// Visit all reachable nodes (BFS).
pub fn count_reachable(start: &NodeRef) -> usize {
    use std::collections::{HashSet, VecDeque};

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(Rc::clone(start));
    visited.insert(start.borrow().id);

    let mut count = 0;
    while let Some(node) = queue.pop_front() {
        count += 1;
        for neighbor in &node.borrow().edges {
            let id = neighbor.borrow().id;
            if visited.insert(id) {
                queue.push_back(Rc::clone(neighbor));
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = GraphNode::new(1, 42);
        assert_eq!(node.borrow().id, 1);
        assert_eq!(node.borrow().value, 42);
        assert_eq!(GraphNode::degree(&node), 0);
    }

    #[test]
    fn test_add_edge() {
        let a = GraphNode::new(1, 10);
        let b = GraphNode::new(2, 20);
        GraphNode::add_edge(&a, &b);

        assert_eq!(GraphNode::degree(&a), 1);
        assert!(GraphNode::has_edge_to(&a, 2));
    }

    #[test]
    fn test_bidirectional() {
        let a = GraphNode::new(1, 10);
        let b = GraphNode::new(2, 20);
        GraphNode::add_bidirectional(&a, &b);

        assert!(GraphNode::has_edge_to(&a, 2));
        assert!(GraphNode::has_edge_to(&b, 1));
    }

    #[test]
    fn test_simple_graph() {
        let (a, b, c) = create_simple_graph();

        assert_eq!(GraphNode::degree(&a), 1);
        assert_eq!(GraphNode::degree(&b), 2);
        assert_eq!(GraphNode::degree(&c), 1);

        assert!(GraphNode::has_edge_to(&a, 2));
        assert!(GraphNode::has_edge_to(&b, 1));
        assert!(GraphNode::has_edge_to(&b, 3));
        assert!(GraphNode::has_edge_to(&c, 2));
    }

    #[test]
    fn test_sum_neighbor_values() {
        let (a, b, c) = create_simple_graph();

        assert_eq!(sum_neighbor_values(&a), 20); // B
        assert_eq!(sum_neighbor_values(&b), 40); // A + C
        assert_eq!(sum_neighbor_values(&c), 20); // B
    }

    #[test]
    fn test_modify_through_graph() {
        let (a, b, _c) = create_simple_graph();

        // Modify B's value through A's edge
        a.borrow().edges[0].borrow_mut().value = 100;

        // Change visible through B
        assert_eq!(b.borrow().value, 100);
    }

    #[test]
    fn test_parent_weak_ref() {
        let parent = GraphNode::new(1, 10);
        let child = GraphNode::new(2, 20);

        GraphNode::set_parent(&child, &parent);

        // Child has weak ref to parent
        assert!(child.borrow().parent.is_some());

        // Can upgrade to strong ref
        let parent_ref = child.borrow().parent.as_ref().unwrap().upgrade();
        assert!(parent_ref.is_some());
        assert_eq!(parent_ref.unwrap().borrow().id, 1);
    }

    #[test]
    fn test_count_reachable() {
        let (a, _b, _c) = create_simple_graph();
        assert_eq!(count_reachable(&a), 3);
    }

    #[test]
    fn test_disconnected_graph() {
        let a = GraphNode::new(1, 10);
        let b = GraphNode::new(2, 20);
        // No edges between them
        assert_eq!(count_reachable(&a), 1);
        assert_eq!(count_reachable(&b), 1);
    }

    #[test]
    fn test_complex_graph() {
        let a = GraphNode::new(1, 1);
        let b = GraphNode::new(2, 2);
        let c = GraphNode::new(3, 3);
        let d = GraphNode::new(4, 4);

        GraphNode::add_edge(&a, &b);
        GraphNode::add_edge(&a, &c);
        GraphNode::add_edge(&b, &d);
        GraphNode::add_edge(&c, &d);

        assert_eq!(count_reachable(&a), 4);
        assert_eq!(GraphNode::degree(&a), 2);
        assert_eq!(GraphNode::degree(&d), 0);
    }
}
