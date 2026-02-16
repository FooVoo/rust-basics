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
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Create a node with left and right children.
    pub fn with_children(value: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode { value, left, right }
    }

    /// Count the total number of nodes in the tree.
    pub fn count_nodes(&self) -> usize {
        let left_count = self.left.as_ref().map_or(0, |node| node.count_nodes());
        let right_count = self.right.as_ref().map_or(0, |node| node.count_nodes());
        1 + left_count + right_count
    }

    /// Calculate the sum of all values in the tree.
    pub fn sum(&self) -> i32 {
        let left_sum = self.left.as_ref().map_or(0, |node| node.sum());
        let right_sum = self.right.as_ref().map_or(0, |node| node.sum());
        self.value + left_sum + right_sum
    }

    /// Get the maximum depth of the tree.
    pub fn max_depth(&self) -> usize {
        let left_depth = self.left.as_ref().map_or(0, |node| node.max_depth());
        let right_depth = self.right.as_ref().map_or(0, |node| node.max_depth());
        1 + left_depth.max(right_depth)
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
