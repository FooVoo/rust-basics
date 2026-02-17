//! Exercise 28: Specialization Patterns - Advanced trait implementation patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use trait implementation patterns for specialization
//! - Implement traits with different behaviors for different types
//! - Understand trait coherence and overlapping implementations

use std::fmt::Display;

pub trait Serializer {
    fn serialize(&self) -> String;
}

// Default implementation for types implementing Display
impl<T: Display> Serializer for T {
    fn serialize(&self) -> String {
        todo!("Implement serialize")
    }
}

// Newtype pattern to provide specialized implementation
pub struct JsonValue<T>(pub T);

impl<T: Display> Serializer for JsonValue<T> {
    fn serialize(&self) -> String {
        todo!("Implement serialize")
    }
}

pub struct XmlValue<T>(pub T);

impl<T: Display> Serializer for XmlValue<T> {
    fn serialize(&self) -> String {
        todo!("Implement serialize")
    }
}

// Trait for optimization hints
pub trait Optimizable {
    fn can_optimize(&self) -> bool {
        todo!("Implement can_optimize")
    }
    
    fn optimize(&mut self) -> String {
        todo!("Implement optimize")
    }
}

// Specialized for Vec
impl<T> Optimizable for Vec<T> {
    fn can_optimize(&self) -> bool {
        todo!("Implement can_optimize")
    }
    
    fn optimize(&mut self) -> String {
        todo!("Implement optimize")
    }
}

// Specialized for String
impl Optimizable for String {
    fn can_optimize(&self) -> bool {
        todo!("Implement can_optimize")
    }
    
    fn optimize(&mut self) -> String {
        todo!("Implement optimize")
    }
}

// Pattern: Use different traits for different behavior
pub trait FastPath {
    fn fast_process(&self) -> String;
}

pub trait SlowPath {
    fn slow_process(&self) -> String;
}

pub trait Process: FastPath + SlowPath {
    fn process(&self, use_fast: bool) -> String {
        todo!("Implement process")
    }
}

pub struct DataProcessor {
    pub data: Vec<i32>,
}

impl FastPath for DataProcessor {
    fn fast_process(&self) -> String {
        todo!("Implement fast_process")
    }
}

impl SlowPath for DataProcessor {
    fn slow_process(&self) -> String {
        todo!("Implement slow_process")
    }
}

impl Process for DataProcessor {}

// Pattern: Marker traits for specialization
pub trait Simple {}
pub trait Complex {}

pub struct SimpleType;
impl Simple for SimpleType {}

pub struct ComplexType;
impl Complex for ComplexType {}

pub trait Handler {
    fn handle(&self) -> String;
}

impl Handler for SimpleType {
    fn handle(&self) -> String {
        todo!("Implement handle")
    }
}

impl Handler for ComplexType {
    fn handle(&self) -> String {
        todo!("Implement handle")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_serializer() {
        let num = 42;
        assert_eq!(num.serialize(), "Display: 42");
    }

    #[test]
    fn test_json_value_serializer() {
        let json = JsonValue(42);
        assert!(json.serialize().contains("{\"value\":\"42\"}"));
    }

    #[test]
    fn test_xml_value_serializer() {
        let xml = XmlValue("test");
        assert!(xml.serialize().contains("<value>test</value>"));
    }

    #[test]
    fn test_json_different_types() {
        let json_int = JsonValue(100);
        let json_str = JsonValue("hello");
        
        assert!(json_int.serialize().contains("100"));
        assert!(json_str.serialize().contains("hello"));
    }

    #[test]
    fn test_vec_optimizable() {
        let mut vec = Vec::with_capacity(100);
        vec.push(1);
        vec.push(2);
        
        assert!(vec.can_optimize());
        let result = vec.optimize();
        assert!(result.contains("Optimized"));
    }

    #[test]
    fn test_string_optimizable() {
        let mut s = String::with_capacity(100);
        s.push_str("test");
        
        assert!(s.can_optimize());
        let result = s.optimize();
        assert!(result.contains("String optimized"));
    }

    #[test]
    fn test_data_processor_fast() {
        let processor = DataProcessor {
            data: vec![1, 2, 3, 4, 5],
        };
        
        let result = processor.fast_process();
        assert!(result.contains("Fast"));
        assert!(result.contains("5 items"));
    }

    #[test]
    fn test_data_processor_slow() {
        let processor = DataProcessor {
            data: vec![1, 2, 3, 4, 5],
        };
        
        let result = processor.slow_process();
        assert!(result.contains("Slow"));
        assert!(result.contains("15")); // sum
    }

    #[test]
    fn test_data_processor_process() {
        let processor = DataProcessor {
            data: vec![10, 20, 30],
        };
        
        let fast = processor.process(true);
        let slow = processor.process(false);
        
        assert!(fast.contains("Fast"));
        assert!(slow.contains("60")); // sum
    }

    #[test]
    fn test_simple_handler() {
        let simple = SimpleType;
        assert_eq!(simple.handle(), "Simple handling");
    }

    #[test]
    fn test_complex_handler() {
        let complex = ComplexType;
        assert_eq!(complex.handle(), "Complex handling");
    }

    #[test]
    fn test_newtype_pattern() {
        let value = 123;
        let json = JsonValue(value);
        let xml = XmlValue(value);
        
        // Same underlying value, different serialization
        assert!(json.serialize().contains("value"));
        assert!(xml.serialize().contains("value"));
        assert!(json.serialize().contains("{"));
        assert!(xml.serialize().contains("<"));
    }

    #[test]
    fn test_optimization_pattern() {
        let mut vec: Vec<i32> = Vec::with_capacity(1000);
        for i in 0..10 {
            vec.push(i);
        }
        
        assert!(vec.can_optimize());
        vec.optimize();
        // After optimization, capacity should be closer to length
    }

    #[test]
    fn test_fast_slow_paths() {
        let small_processor = DataProcessor {
            data: vec![1, 2],
        };
        let large_processor = DataProcessor {
            data: (1..=1000).collect(),
        };
        
        // Can choose path based on data size
        let _ = small_processor.process(false); // Use slow path for accuracy
        let _ = large_processor.process(true);  // Use fast path for speed
    }

    #[test]
    fn test_different_serializations() {
        let data = "test data";
        let json = JsonValue(data);
        let xml = XmlValue(data);
        
        let json_str = json.serialize();
        let xml_str = xml.serialize();
        
        assert!(json_str.contains("test data"));
        assert!(xml_str.contains("test data"));
        assert_ne!(json_str, xml_str);
    }
}
