//! Exercise 27: Advanced Associated Types - Complex associated type patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use multiple associated types
//! - Constrain associated types with bounds
//! - Create complex trait relationships with associated types

use std::fmt::Display;

pub trait Converter {
    type Input;
    type Output;
    type Error;
    
    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

pub struct StringToIntConverter;

impl Converter for StringToIntConverter {
    type Input = String;
    type Output = i32;
    type Error = String;
    
    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>  {
        todo!("Implement convert")
    }
}

pub struct IntToStringConverter;

impl Converter for IntToStringConverter {
    type Input = i32;
    type Output = String;
    type Error = ();
    
    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>  {
        todo!("Implement convert")
    }
}

// Chain converters
pub fn chain_convert<C1, C2>(
    converter1: &C1,
    converter2: &C2,
    input: C1::Input,
) -> Result<C2::Output, String>
where
    C1: Converter,
    C2: Converter<Input = C1::Output>,
    C1::Error: Display,
    C2::Error: Display,
 {
    todo!("Implement chain_convert")
}

pub trait Collection {
    type Item;
    type Iterator: Iterator<Item = Self::Item>;
    
    fn iter(&self) -> Self::Iterator;
    fn len(&self) -> usize;
}

pub struct SimpleVec<T> {
    data: Vec<T>,
}

impl<T: Clone> Collection for SimpleVec<T> {
    type Item = T;
    type Iterator = std::vec::IntoIter<T>;
    
    fn iter(&self) -> Self::Iterator  {
        todo!("Implement iter")
    }
    
    fn len(&self) -> usize  {
        todo!("Implement len")
    }
}

impl<T> SimpleVec<T> {
    pub fn new() -> Self  {
        todo!("Implement new")
    }
    
    pub fn push(&mut self, item: T)  {
        todo!("Implement push")
    }
}

pub trait Graph {
    type Node;
    type Edge;
    type NodeIter: Iterator<Item = Self::Node>;
    type EdgeIter: Iterator<Item = Self::Edge>;
    
    fn nodes(&self) -> Self::NodeIter;
    fn edges(&self) -> Self::EdgeIter;
}

pub struct SimpleGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl SimpleGraph {
    pub fn new() -> Self  {
        todo!("Implement new")
    }
    
    pub fn add_node(&mut self, name: String)  {
        todo!("Implement add_node")
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize)  {
        todo!("Implement add_edge")
    }
}

impl Graph for SimpleGraph {
    type Node = String;
    type Edge = (usize, usize);
    type NodeIter = std::vec::IntoIter<String>;
    type EdgeIter = std::vec::IntoIter<(usize, usize)>;
    
    fn nodes(&self) -> Self::NodeIter  {
        todo!("Implement nodes")
    }
    
    fn edges(&self) -> Self::EdgeIter  {
        todo!("Implement edges")
    }
}

// Function using associated types with bounds
pub fn process_collection<C>(collection: &C) -> Vec<String>
where
    C: Collection,
    C::Item: Display,
 {
    todo!("Implement process_collection")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_int_converter() {
        let converter = StringToIntConverter;
        assert_eq!(converter.convert("42".to_string()), Ok(42));
        assert!(converter.convert("abc".to_string()).is_err());
    }

    #[test]
    fn test_int_to_string_converter() {
        let converter = IntToStringConverter;
        assert_eq!(converter.convert(42), Ok("42".to_string()));
        assert_eq!(converter.convert(-10), Ok("-10".to_string()));
    }

    #[test]
    fn test_simple_vec_collection() {
        let mut vec = SimpleVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        let items: Vec<i32> = vec.iter().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_simple_graph() {
        let mut graph = SimpleGraph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_edge(0, 1);
        
        let nodes: Vec<String> = graph.nodes().collect();
        assert_eq!(nodes.len(), 2);
        
        let edges: Vec<(usize, usize)> = graph.edges().collect();
        assert_eq!(edges.len(), 1);
    }

    #[test]
    fn test_process_collection() {
        let mut vec = SimpleVec::new();
        vec.push(10);
        vec.push(20);
        vec.push(30);
        
        let strings = process_collection(&vec);
        assert_eq!(strings, vec!["10", "20", "30"]);
    }

    #[test]
    fn test_multiple_associated_types() {
        let converter = StringToIntConverter;
        
        // Input type is String
        let input = "100".to_string();
        // Output type is i32
        let output: Result<i32, String> = converter.convert(input);
        
        assert_eq!(output, Ok(100));
    }

    #[test]
    fn test_graph_nodes_iteration() {
        let mut graph = SimpleGraph::new();
        graph.add_node("Node1".to_string());
        graph.add_node("Node2".to_string());
        graph.add_node("Node3".to_string());
        
        let node_count = graph.nodes().count();
        assert_eq!(node_count, 3);
    }

    #[test]
    fn test_graph_edges_iteration() {
        let mut graph = SimpleGraph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        
        let edges: Vec<_> = graph.edges().collect();
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0], (0, 1));
        assert_eq!(edges[1], (1, 2));
    }

    #[test]
    fn test_associated_type_bounds() {
        let mut vec = SimpleVec::new();
        vec.push("hello");
        vec.push("world");
        
        // process_collection requires C::Item: Display
        let result = process_collection(&vec);
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_complex_associated_types() {
        let converter1 = StringToIntConverter;
        let converter2 = IntToStringConverter;
        
        // Type checking ensures the pipeline is valid
        let result1 = converter1.convert("42".to_string());
        assert_eq!(result1, Ok(42));
        
        if let Ok(intermediate) = result1 {
            let result2 = converter2.convert(intermediate);
            assert_eq!(result2, Ok("42".to_string()));
        }
    }

    #[test]
    fn test_iterator_associated_type() {
        let mut vec = SimpleVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        // The Iterator associated type is properly bounded
        let doubled: Vec<_> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }
}
