//! Exercise 25: Self-Referential Patterns - Managing internal references
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand challenges with self-referential structs
//! - Use safe alternatives to self-references
//! - Work with indices and IDs instead of references

pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        todo!("Implement new")
    }
    
    pub fn add(&mut self, item: T) -> usize {
        todo!("Implement add")
    }
    
    pub fn get(&self, id: usize) -> Option<&T> {
        todo!("Implement get")
    }
    
    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        todo!("Implement get_mut")
    }
    
    pub fn len(&self) -> usize {
        todo!("Implement len")
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub value: i32,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl TreeNode {
    pub fn new(value: i32, parent: Option<usize>) -> Self {
        todo!("Implement new")
    }
}

pub struct Tree {
    arena: Arena<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        todo!("Implement new")
    }
    
    pub fn add_node(&mut self, value: i32, parent: Option<usize>) -> usize {
        todo!("Implement add_node")
    }
    
    pub fn get_node(&self, id: usize) -> Option<&TreeNode> {
        todo!("Implement get_node")
    }
    
    pub fn node_count(&self) -> usize {
        todo!("Implement node_count")
    }
}

/// Build a simple tree and return its depth.
pub fn build_and_measure_tree() -> usize {
    todo!("Implement build_and_measure_tree")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_new() {
        let arena: Arena<i32> = Arena::new();
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn test_arena_add_and_get() {
        let mut arena = Arena::new();
        let id1 = arena.add(10);
        let id2 = arena.add(20);
        
        assert_eq!(arena.get(id1), Some(&10));
        assert_eq!(arena.get(id2), Some(&20));
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn test_tree_creation() {
        let tree = Tree::new();
        assert_eq!(tree.node_count(), 0);
    }

    #[test]
    fn test_tree_add_nodes() {
        let mut tree = Tree::new();
        let root = tree.add_node(1, None);
        let child = tree.add_node(2, Some(root));
        
        assert_eq!(tree.node_count(), 2);
        
        let root_node = tree.get_node(root).unwrap();
        assert_eq!(root_node.value, 1);
        assert_eq!(root_node.children, vec![child]);
        
        let child_node = tree.get_node(child).unwrap();
        assert_eq!(child_node.parent, Some(root));
    }

    #[test]
    fn test_build_and_measure_tree() {
        let depth = build_and_measure_tree();
        assert_eq!(depth, 3); // root -> child -> grandchild
    }

    #[test]
    fn test_arena_mutation() {
        let mut arena = Arena::new();
        let id = arena.add(42);
        
        if let Some(item) = arena.get_mut(id) {
            *item = 100;
        }
        
        assert_eq!(arena.get(id), Some(&100));
    }
}
