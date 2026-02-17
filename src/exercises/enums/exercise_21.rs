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
        todo!("Implement empty")
    }

    /// Creates a leaf node
    pub fn leaf(value: T) -> Self {
        todo!("Implement leaf")
    }

    /// Creates a node with children
    pub fn node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
        todo!("Implement node")
    }

    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        todo!("Implement height")
    }

    /// Counts the number of nodes in the tree
    pub fn size(&self) -> usize {
        todo!("Implement size")
    }
}

impl BinaryTree<i32> {
    /// Returns the sum of all values in the tree
    pub fn sum(&self) -> i32 {
        todo!("Implement sum")
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
