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
        Arena { items: Vec::new() }
    }
    
    pub fn add(&mut self, item: T) -> usize {
        let id = self.items.len();
        self.items.push(item);
        id
    }
    
    pub fn get(&self, id: usize) -> Option<&T> {
        self.items.get(id)
    }
    
    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.items.get_mut(id)
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
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
        TreeNode {
            value,
            parent,
            children: Vec::new(),
        }
    }
}

pub struct Tree {
    arena: Arena<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            arena: Arena::new(),
        }
    }
    
    pub fn add_node(&mut self, value: i32, parent: Option<usize>) -> usize {
        let id = self.arena.add(TreeNode::new(value, parent));
        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.arena.get_mut(parent_id) {
                parent_node.children.push(id);
            }
        }
        id
    }
    
    pub fn get_node(&self, id: usize) -> Option<&TreeNode> {
        self.arena.get(id)
    }
    
    pub fn node_count(&self) -> usize {
        self.arena.len()
    }
}

/// Build a simple tree and return its depth.
pub fn build_and_measure_tree() -> usize {
    let mut tree = Tree::new();
    let root = tree.add_node(1, None);
    let child1 = tree.add_node(2, Some(root));
    let _child2 = tree.add_node(3, Some(root));
    let _grandchild = tree.add_node(4, Some(child1));
    
    // Calculate depth
    let mut max_depth = 0;
    for i in 0..tree.node_count() {
        let mut depth = 0;
        let mut current = Some(i);
        while let Some(id) = current {
            if let Some(node) = tree.get_node(id) {
                depth += 1;
                current = node.parent;
            } else {
                break;
            }
        }
        max_depth = max_depth.max(depth);
    }
    max_depth
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
