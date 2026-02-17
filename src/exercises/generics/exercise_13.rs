//! Exercise 13: Generic Tree Structure - Build a generic tree
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create recursive generic structures
//! - Work with Box for recursive types
//! - Implement tree operations generically

/// A generic binary tree node.
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    /// Creates a new leaf node.
    pub fn leaf(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Creates a new node with children.
    pub fn with_children(value: T, left: Option<TreeNode<T>>, right: Option<TreeNode<T>>) -> Self {
        TreeNode {
            value,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Checks if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Counts the total nodes in the tree.
    pub fn count_nodes(&self) -> usize {
        let left_count = self.left.as_ref().map_or(0, |node| node.count_nodes());
        let right_count = self.right.as_ref().map_or(0, |node| node.count_nodes());
        1 + left_count + right_count
    }

    /// Gets the height of the tree.
    pub fn height(&self) -> usize {
        let left_height = self.left.as_ref().map_or(0, |node| node.height());
        let right_height = self.right.as_ref().map_or(0, |node| node.height());
        1 + left_height.max(right_height)
    }
}

impl<T: Clone> TreeNode<T> {
    /// Collects all values in pre-order.
    pub fn preorder(&self) -> Vec<T> {
        let mut result = vec![self.value.clone()];
        if let Some(ref left) = self.left {
            result.extend(left.preorder());
        }
        if let Some(ref right) = self.right {
            result.extend(right.preorder());
        }
        result
    }
}

impl<T: PartialEq> TreeNode<T> {
    /// Searches for a value in the tree.
    pub fn contains(&self, value: &T) -> bool {
        if self.value == *value {
            return true;
        }
        let in_left = self.left.as_ref().map_or(false, |node| node.contains(value));
        let in_right = self.right.as_ref().map_or(false, |node| node.contains(value));
        in_left || in_right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_creation() {
        let leaf = TreeNode::leaf(42);
        assert_eq!(leaf.value, 42);
        assert!(leaf.is_leaf());
    }

    #[test]
    fn test_with_children() {
        let left = TreeNode::leaf(1);
        let right = TreeNode::leaf(2);
        let root = TreeNode::with_children(0, Some(left), Some(right));
        assert_eq!(root.value, 0);
        assert!(!root.is_leaf());
    }

    #[test]
    fn test_count_nodes_leaf() {
        let leaf = TreeNode::leaf(1);
        assert_eq!(leaf.count_nodes(), 1);
    }

    #[test]
    fn test_count_nodes_tree() {
        let left = TreeNode::leaf(1);
        let right = TreeNode::leaf(2);
        let root = TreeNode::with_children(0, Some(left), Some(right));
        assert_eq!(root.count_nodes(), 3);
    }

    #[test]
    fn test_height_leaf() {
        let leaf = TreeNode::leaf(1);
        assert_eq!(leaf.height(), 1);
    }

    #[test]
    fn test_height_tree() {
        let left = TreeNode::leaf(1);
        let right = TreeNode::with_children(2, Some(TreeNode::leaf(3)), None);
        let root = TreeNode::with_children(0, Some(left), Some(right));
        assert_eq!(root.height(), 3);
    }

    #[test]
    fn test_preorder() {
        let left = TreeNode::leaf(1);
        let right = TreeNode::leaf(2);
        let root = TreeNode::with_children(0, Some(left), Some(right));
        assert_eq!(root.preorder(), vec![0, 1, 2]);
    }

    #[test]
    fn test_contains_found() {
        let left = TreeNode::leaf(1);
        let right = TreeNode::leaf(2);
        let root = TreeNode::with_children(0, Some(left), Some(right));
        assert!(root.contains(&1));
        assert!(root.contains(&2));
    }

    #[test]
    fn test_contains_not_found() {
        let leaf = TreeNode::leaf(1);
        assert!(!leaf.contains(&2));
    }
}
