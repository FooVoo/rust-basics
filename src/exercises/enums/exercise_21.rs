//! Exercise 21: Binary Tree - Recursive Enum Structure
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create recursive enum types with Box
//! - Implement tree data structures
//! - Work with complex recursive patterns

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    /// Creates an empty tree
    pub fn empty() -> Self {
        BinaryTree::Empty
    }

    /// Creates a leaf node
    pub fn leaf(value: T) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }
    }

    /// Creates a node with children
    pub fn node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        match self {
            BinaryTree::Empty => 0,
            BinaryTree::Node { left, right, .. } => {
                1 + std::cmp::max(left.height(), right.height())
            }
        }
    }

    /// Counts the number of nodes in the tree
    pub fn size(&self) -> usize {
        match self {
            BinaryTree::Empty => 0,
            BinaryTree::Node { left, right, .. } => 1 + left.size() + right.size(),
        }
    }
}

impl BinaryTree<i32> {
    /// Returns the sum of all values in the tree
    pub fn sum(&self) -> i32 {
        match self {
            BinaryTree::Empty => 0,
            BinaryTree::Node { value, left, right } => value + left.sum() + right.sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: BinaryTree<i32> = BinaryTree::empty();
        assert_eq!(tree.height(), 0);
        assert_eq!(tree.size(), 0);
        assert_eq!(tree.sum(), 0);
    }

    #[test]
    fn test_leaf_node() {
        let tree = BinaryTree::leaf(5);
        assert_eq!(tree.height(), 1);
        assert_eq!(tree.size(), 1);
        assert_eq!(tree.sum(), 5);
    }

    #[test]
    fn test_complex_tree() {
        let tree = BinaryTree::node(
            10,
            BinaryTree::node(5, BinaryTree::leaf(3), BinaryTree::leaf(7)),
            BinaryTree::leaf(15),
        );
        assert_eq!(tree.height(), 3);
        assert_eq!(tree.size(), 5);
        assert_eq!(tree.sum(), 40);
    }
}
