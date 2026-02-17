//! Exercise 21: Associated Types - Use associated types in traits
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Define and use associated types
//! - Understand when to use associated types vs generic parameters
//! - Implement traits with associated types

pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while self.next().is_some() {
            count += 1;
        }
        count
    }
}

pub struct RangeIterator {
    current: i32,
    end: i32,
}

impl RangeIterator {
    pub fn new(start: i32, end: i32) -> Self {
        RangeIterator { current: start, end }
    }
}

impl Iterator for RangeIterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub trait Graph {
    type Node;
    type Edge;
    
    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self) -> Vec<Self::Edge>;
    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, edge: Self::Edge);
}

pub struct SimpleGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl SimpleGraph {
    pub fn new() -> Self {
        SimpleGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl Graph for SimpleGraph {
    type Node = String;
    type Edge = (usize, usize);
    
    fn nodes(&self) -> Vec<Self::Node> {
        self.nodes.clone()
    }
    
    fn edges(&self) -> Vec<Self::Edge> {
        self.edges.clone()
    }
    
    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }
    
    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }
}

pub trait Container {
    type Item;
    
    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

impl<T> Container for Vec<T> {
    type Item = T;
    
    fn add(&mut self, item: Self::Item) {
        self.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        <[T]>::get(self, index)
    }
    
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_iterator_basic() {
        let mut iter = RangeIterator::new(0, 3);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_range_iterator_count() {
        let iter = RangeIterator::new(5, 10);
        assert_eq!(iter.count(), 5);
    }

    #[test]
    fn test_range_iterator_empty() {
        let mut iter = RangeIterator::new(5, 5);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_simple_graph_nodes() {
        let mut graph = SimpleGraph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        
        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.nodes()[0], "A");
    }

    #[test]
    fn test_simple_graph_edges() {
        let mut graph = SimpleGraph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_edge((0, 1));
        
        assert_eq!(graph.edges().len(), 1);
        assert_eq!(graph.edges()[0], (0, 1));
    }

    #[test]
    fn test_simple_graph_complex() {
        let mut graph = SimpleGraph::new();
        graph.add_node("Node1".to_string());
        graph.add_node("Node2".to_string());
        graph.add_node("Node3".to_string());
        graph.add_edge((0, 1));
        graph.add_edge((1, 2));
        
        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 2);
    }

    #[test]
    fn test_vec_container() {
        let mut container: Vec<i32> = Vec::new();
        container.add(10);
        container.add(20);
        container.add(30);
        
        assert_eq!(container.len(), 3);
        assert_eq!(container.get(0), Some(&10));
        assert_eq!(container.get(1), Some(&20));
    }

    #[test]
    fn test_vec_container_strings() {
        let mut container: Vec<String> = Vec::new();
        container.add("hello".to_string());
        container.add("world".to_string());
        
        assert_eq!(container.len(), 2);
        assert_eq!(container.get(0).unwrap(), "hello");
    }

    #[test]
    fn test_associated_type_inference() {
        let mut iter = RangeIterator::new(1, 4);
        // The type Item = i32 is inferred from the implementation
        let first: i32 = iter.next().unwrap();
        assert_eq!(first, 1);
    }

    #[test]
    fn test_generic_over_graph() {
        fn count_nodes<G: Graph>(graph: &G) -> usize {
            graph.nodes().len()
        }
        
        let mut graph = SimpleGraph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        
        assert_eq!(count_nodes(&graph), 2);
    }
}
