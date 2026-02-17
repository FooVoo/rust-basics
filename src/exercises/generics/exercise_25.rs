//! Exercise 25: Generic Graph Structure - Build a complex generic graph
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create complex generic data structures
//! - Work with adjacency lists
//! - Implement graph algorithms generically

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A generic directed graph.
pub struct Graph<T>
where
    T: Eq + Hash + Clone,
{
    adjacency: HashMap<T, Vec<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Graph {
            adjacency: HashMap::new(),
        }
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: T) {
        self.adjacency.entry(node).or_insert_with(Vec::new);
    }

    /// Adds a directed edge from source to destination.
    pub fn add_edge(&mut self, from: T, to: T) {
        self.add_node(from.clone());
        self.add_node(to.clone());
        self.adjacency.get_mut(&from).unwrap().push(to);
    }

    /// Gets all neighbors of a node.
    pub fn neighbors(&self, node: &T) -> Option<&Vec<T>> {
        self.adjacency.get(node)
    }

    /// Checks if a node exists in the graph.
    pub fn contains(&self, node: &T) -> bool {
        self.adjacency.contains_key(node)
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Performs a depth-first search starting from a node.
    pub fn dfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_helper(start, &mut visited, &mut result);
        result
    }

    fn dfs_helper(&self, node: &T, visited: &mut HashSet<T>, result: &mut Vec<T>) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node.clone());
        result.push(node.clone());

        if let Some(neighbors) = self.neighbors(node) {
            for neighbor in neighbors {
                self.dfs_helper(neighbor, visited, result);
            }
        }
    }

    /// Checks if there's a path from source to destination.
    pub fn has_path(&self, from: &T, to: &T) -> bool {
        let mut visited = HashSet::new();
        self.has_path_helper(from, to, &mut visited)
    }

    fn has_path_helper(&self, current: &T, target: &T, visited: &mut HashSet<T>) -> bool {
        if current == target {
            return true;
        }
        if visited.contains(current) {
            return false;
        }
        visited.insert(current.clone());

        if let Some(neighbors) = self.neighbors(current) {
            for neighbor in neighbors {
                if self.has_path_helper(neighbor, target, visited) {
                    return true;
                }
            }
        }
        false
    }
}

impl<T> Default for Graph<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let graph: Graph<i32> = Graph::new();
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        assert!(graph.contains(&1));
        assert!(graph.contains(&2));
    }

    #[test]
    fn test_neighbors() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        let neighbors = graph.neighbors(&1).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        let visited = graph.dfs(&1);
        assert_eq!(visited.len(), 4);
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
        assert!(visited.contains(&3));
        assert!(visited.contains(&4));
    }

    #[test]
    fn test_dfs_string_nodes() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("A".to_string(), "C".to_string());
        graph.add_edge("B".to_string(), "D".to_string());
        let visited = graph.dfs(&"A".to_string());
        assert_eq!(visited.len(), 4);
    }

    #[test]
    fn test_has_path_true() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        assert!(graph.has_path(&1, &3));
    }

    #[test]
    fn test_has_path_false() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        assert!(!graph.has_path(&1, &4));
    }

    #[test]
    fn test_has_path_self() {
        let mut graph = Graph::new();
        graph.add_node(1);
        assert!(graph.has_path(&1, &1));
    }

    #[test]
    fn test_graph_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        assert!(graph.has_path(&1, &1));
    }
}
