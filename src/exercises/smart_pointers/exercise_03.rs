//! Exercise 03: Binary Tree - Build a binary tree using Box
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Create complex recursive structures with Box
//! - Implement tree traversal
//! - Work with nested Box pointers

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    /// Create a new leaf node.
    pub fn new(value: i32) -> Self  {
        todo!("Create a new leaf node.")
    }

    /// Create a node with left and right children.
    pub fn with_children(value: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self  {
        todo!("Create a node with left and right children.")
    }

    /// Count the total number of nodes in the tree.
    pub fn count_nodes(&self) -> usize  {
        todo!("Count the total number of nodes in the tree.")
    }

    /// Calculate the sum of all values in the tree.
    pub fn sum(&self) -> i32  {
        todo!("Calculate the sum of all values in the tree.")
    }

    /// Get the maximum depth of the tree.
    pub fn max_depth(&self) -> usize  {
        todo!("Get the maximum depth of the tree.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_node() {
        let node = TreeNode::new(42);
        assert_eq!(node.value, 42);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
        assert_eq!(node.count_nodes(), 1);
        assert_eq!(node.sum(), 42);
        assert_eq!(node.max_depth(), 1);
    }

    #[test]
    fn test_tree_with_children() {
        let tree = TreeNode::with_children(
            1,
            Some(Box::new(TreeNode::new(2))),
            Some(Box::new(TreeNode::new(3))),
        );
        assert_eq!(tree.count_nodes(), 3);
        assert_eq!(tree.sum(), 6);
        assert_eq!(tree.max_depth(), 2);
    }

    #[test]
    fn test_complex_tree() {
        let tree = TreeNode::with_children(
            10,
            Some(Box::new(TreeNode::with_children(
                5,
                Some(Box::new(TreeNode::new(3))),
                Some(Box::new(TreeNode::new(7))),
            ))),
            Some(Box::new(TreeNode::new(15))),
        );
        assert_eq!(tree.count_nodes(), 5);
        assert_eq!(tree.sum(), 40);
        assert_eq!(tree.max_depth(), 3);
    }
}
