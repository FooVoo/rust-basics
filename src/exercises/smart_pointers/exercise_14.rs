//! Exercise 14: Weak References - Break reference cycles
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand Weak<T> for breaking cycles
//! - Learn upgrade() to get strong references
//! - Prevent memory leaks with weak references

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub parent: RefCell<Weak<TreeNode>>,
    pub children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    /// Create a new tree node.
    pub fn new(value: i32) -> Rc<Self> {
        todo!("Implement new")
    }

    /// Add a child to this node.
    pub fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        todo!("Implement add_child")
    }

    /// Get parent value if it exists.
    pub fn parent_value(node: &Rc<TreeNode>) -> Option<i32> {
        todo!("Implement parent_value")
    }

    /// Count children.
    pub fn child_count(node: &Rc<TreeNode>) -> usize {
        todo!("Implement child_count")
    }

    /// Get strong and weak counts.
    pub fn counts(node: &Rc<TreeNode>) -> (usize, usize) {
        todo!("Implement counts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = TreeNode::new(42);
        assert_eq!(node.value, 42);
        assert_eq!(TreeNode::child_count(&node), 0);
        assert!(TreeNode::parent_value(&node).is_none());
    }

    #[test]
    fn test_add_child() {
        let parent = TreeNode::new(1);
        let child = TreeNode::new(2);
        TreeNode::add_child(&parent, child.clone());
        
        assert_eq!(TreeNode::child_count(&parent), 1);
        assert_eq!(TreeNode::parent_value(&child), Some(1));
    }

    #[test]
    fn test_weak_reference() {
        let parent = TreeNode::new(1);
        let child = TreeNode::new(2);
        
        TreeNode::add_child(&parent, child.clone());
        
        // Parent has strong ref to child
        let (strong, _weak) = TreeNode::counts(&child);
        assert_eq!(strong, 2); // original + parent
        
        // Child has weak ref to parent (doesn't increase strong count)
        let (strong, weak) = TreeNode::counts(&parent);
        assert_eq!(strong, 1); // only original
        assert_eq!(weak, 1); // weak ref from child
    }

    #[test]
    fn test_parent_dropped() {
        let child = {
            let parent = TreeNode::new(1);
            let child = TreeNode::new(2);
            TreeNode::add_child(&parent, child.clone());
            child
        }; // parent dropped here
        
        // Parent is gone, so upgrade fails
        assert!(TreeNode::parent_value(&child).is_none());
    }

    #[test]
    fn test_multiple_children() {
        let parent = TreeNode::new(1);
        let child1 = TreeNode::new(2);
        let child2 = TreeNode::new(3);
        let child3 = TreeNode::new(4);
        
        TreeNode::add_child(&parent, child1.clone());
        TreeNode::add_child(&parent, child2.clone());
        TreeNode::add_child(&parent, child3.clone());
        
        assert_eq!(TreeNode::child_count(&parent), 3);
        
        // Parent has weak refs from all children
        let (_strong, weak) = TreeNode::counts(&parent);
        assert_eq!(weak, 3);
    }

    #[test]
    fn test_no_cycle() {
        // This test ensures no reference cycle exists
        let parent = TreeNode::new(1);
        let child = TreeNode::new(2);
        TreeNode::add_child(&parent, child);
        
        // When parent goes out of scope, everything should be cleaned up
        // If there was a cycle, this would leak memory
        drop(parent);
        // Test passes if no panic/leak
    }
}
