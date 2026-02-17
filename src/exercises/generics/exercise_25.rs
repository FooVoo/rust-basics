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
        todo!("Implement new")
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: T) {
        todo!("Implement add_node")
    }

    /// Adds a directed edge from source to destination.
    pub fn add_edge(&mut self, from: T, to: T) {
        todo!("Implement add_edge")
    }

    /// Gets all neighbors of a node.
    pub fn neighbors(&self, node: &T) -> Option<&Vec<T>> {
        todo!("Implement neighbors")
    }

    /// Checks if a node exists in the graph.
    pub fn contains(&self, node: &T) -> bool {
        todo!("Implement contains")
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        todo!("Implement node_count")
    }

    /// Performs a depth-first search starting from a node.
    pub fn dfs(&self, start: &T) -> Vec<T> {
        todo!("Implement dfs")
    }

    fn dfs_helper(&self, node: &T, visited: &mut HashSet<T>, result: &mut Vec<T>) {
        todo!("Implement dfs_helper")
    }

    /// Checks if there's a path from source to destination.
    pub fn has_path(&self, from: &T, to: &T) -> bool {
        todo!("Implement has_path")
    }

    fn has_path_helper(&self, current: &T, target: &T, visited: &mut HashSet<T>) -> bool {
        todo!("Implement has_path_helper")
    }
}

impl<T> Default for Graph<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        todo!("Implement default")
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
