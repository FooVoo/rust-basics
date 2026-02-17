//! Exercise 17: Nested Borrowing - Working with nested data structures
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Borrow from nested structures
//! - Navigate through nested references
//! - Manage complex borrowing scenarios

pub struct Node {
    pub value: i32,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(value: i32) -> Self  {
        todo!("Implement new")
    }
    
    pub fn add_child(&mut self, child: Node)  {
        todo!("Implement add_child")
    }
    
    pub fn value(&self) -> i32  {
        todo!("Implement value")
    }
    
    pub fn children(&self) -> &[Node]  {
        todo!("Implement children")
    }
}

/// Sum all values in a tree.
pub fn sum_tree(node: &Node) -> i32  {
    todo!("Sum all values in a tree.")
}

/// Count total nodes in tree.
pub fn count_nodes(node: &Node) -> usize  {
    todo!("Count total nodes in tree.")
}

/// Find maximum value in tree.
pub fn max_value(node: &Node) -> i32  {
    todo!("Find maximum value in tree.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tree() -> Node {
        let mut root = Node::new(1);
        let mut child1 = Node::new(2);
        child1.add_child(Node::new(4));
        child1.add_child(Node::new(5));
        root.add_child(child1);
        root.add_child(Node::new(3));
        root
    }

    #[test]
    fn test_node_creation() {
        let node = Node::new(42);
        assert_eq!(node.value(), 42);
        assert_eq!(node.children().len(), 0);
    }

    #[test]
    fn test_add_child() {
        let mut parent = Node::new(1);
        parent.add_child(Node::new(2));
        assert_eq!(parent.children().len(), 1);
    }

    #[test]
    fn test_sum_tree() {
        let tree = create_test_tree();
        // 1 + 2 + 4 + 5 + 3 = 15
        assert_eq!(sum_tree(&tree), 15);
    }

    #[test]
    fn test_count_nodes() {
        let tree = create_test_tree();
        assert_eq!(count_nodes(&tree), 5);
    }

    #[test]
    fn test_max_value() {
        let tree = create_test_tree();
        assert_eq!(max_value(&tree), 5);
    }

    #[test]
    fn test_single_node() {
        let node = Node::new(100);
        assert_eq!(sum_tree(&node), 100);
        assert_eq!(count_nodes(&node), 1);
        assert_eq!(max_value(&node), 100);
    }
}
