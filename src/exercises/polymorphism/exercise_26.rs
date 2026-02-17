//! Exercise 26: Trait Bounds with Lifetimes - Combine trait bounds and lifetimes
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use trait bounds with lifetime parameters
//! - Understand lifetime bounds in traits
//! - Work with complex lifetime constraints

pub trait Borrowable<'a> {
    type Borrowed;
    fn borrow(&'a self) -> Self::Borrowed;
}

pub struct Container {
    data: String,
}

impl<'a> Borrowable<'a> for Container {
    type Borrowed = &'a str;
    
    fn borrow(&'a self) -> Self::Borrowed  {
        todo!("Implement borrow")
    }
}

pub trait Processor<'a> {
    fn process(&self, input: &'a str) -> String;
}

pub struct UpperProcessor;

impl<'a> Processor<'a> for UpperProcessor {
    fn process(&self, input: &'a str) -> String  {
        todo!("Implement process")
    }
}

pub struct LowerProcessor;

impl<'a> Processor<'a> for LowerProcessor {
    fn process(&self, input: &'a str) -> String  {
        todo!("Implement process")
    }
}

// Function with trait bound and lifetime
pub fn process_with_lifetime<'a, T>(processor: &T, input: &'a str) -> String
where
    T: Processor<'a>,
 {
    todo!("Implement process_with_lifetime")
}

// Trait with lifetime constraint
pub trait Cache<'a> {
    type Item: 'a;
    
    fn get(&self, key: &str) -> Option<&'a Self::Item>;
    fn set(&mut self, key: String, value: &'a Self::Item);
}

use std::collections::HashMap;

pub struct SimpleCache<'a, T: 'a> {
    data: HashMap<String, &'a T>,
}

impl<'a, T: 'a> SimpleCache<'a, T> {
    pub fn new() -> Self  {
        todo!("Implement new")
    }
}

impl<'a, T: 'a> Cache<'a> for SimpleCache<'a, T> {
    type Item = T;
    
    fn get(&self, key: &str) -> Option<&'a Self::Item>  {
        todo!("Implement get")
    }
    
    fn set(&mut self, key: String, value: &'a T)  {
        todo!("Implement set")
    }
}

// Function with multiple lifetime bounds
pub fn compare_and_return<'a, 'b, T>(first: &'a T, second: &'b T, use_first: bool) -> &'a T
where
    T: PartialEq,
    'b: 'a, // 'b outlives 'a
 {
    todo!("Implement compare_and_return")
}

pub trait Visitor<'a> {
    type Output;
    
    fn visit(&mut self, item: &'a str) -> Self::Output;
}

pub struct CountVisitor {
    count: usize,
}

impl<'a> Visitor<'a> for CountVisitor {
    type Output = usize;
    
    fn visit(&mut self, _item: &'a str) -> Self::Output  {
        todo!("Implement visit")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrowable() {
        let container = Container {
            data: "test data".to_string(),
        };
        let borrowed = container.borrow();
        assert_eq!(borrowed, "test data");
    }

    #[test]
    fn test_processor_upper() {
        let processor = UpperProcessor;
        let result = processor.process("hello");
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_processor_lower() {
        let processor = LowerProcessor;
        let result = processor.process("WORLD");
        assert_eq!(result, "world");
    }

    #[test]
    fn test_process_with_lifetime() {
        let processor = UpperProcessor;
        let input = "test";
        let result = process_with_lifetime(&processor, input);
        assert_eq!(result, "TEST");
    }

    #[test]
    fn test_cache_get_set() {
        let value = 42;
        let mut cache = SimpleCache::new();
        
        cache.set("key".to_string(), &value);
        let retrieved = cache.get("key");
        
        assert_eq!(retrieved.map(|v| *v), Some(42));
    }

    #[test]
    fn test_cache_missing() {
        let cache: SimpleCache<i32> = SimpleCache::new();
        assert_eq!(cache.get("nonexistent"), None);
    }

    #[test]
    fn test_compare_and_return() {
        let first = 10;
        let second = 20;
        
        let result = compare_and_return(&first, &second, true);
        assert_eq!(*result, 10);
        
        let result = compare_and_return(&first, &second, false);
        assert_eq!(*result, 10);
    }

    #[test]
    fn test_visitor() {
        let mut visitor = CountVisitor { count: 0 };
        
        assert_eq!(visitor.visit("first"), 1);
        assert_eq!(visitor.visit("second"), 2);
        assert_eq!(visitor.visit("third"), 3);
    }

    #[test]
    fn test_lifetime_with_multiple_items() {
        let container1 = Container {
            data: "first".to_string(),
        };
        let container2 = Container {
            data: "second".to_string(),
        };
        
        let borrowed1 = container1.borrow();
        let borrowed2 = container2.borrow();
        
        assert_eq!(borrowed1, "first");
        assert_eq!(borrowed2, "second");
    }

    #[test]
    fn test_processor_trait_object() {
        let upper = UpperProcessor;
        let lower = LowerProcessor;
        
        // Can't use as trait object directly due to lifetime parameter
        // But can use with concrete lifetime
        let result1 = upper.process("test");
        let result2 = lower.process("TEST");
        
        assert_eq!(result1, "TEST");
        assert_eq!(result2, "test");
    }

    #[test]
    fn test_cache_multiple_types() {
        let val1 = "hello";
        let val2 = "world";
        let mut cache = SimpleCache::new();
        
        cache.set("key1".to_string(), &val1);
        cache.set("key2".to_string(), &val2);
        
        assert_eq!(cache.get("key1").map(|v| *v), Some("hello"));
        assert_eq!(cache.get("key2").map(|v| *v), Some("world"));
    }

    #[test]
    fn test_lifetime_bounds_complex() {
        let data = "test data";
        let processor = UpperProcessor;
        
        // Lifetime of data must outlive the processing
        let result = process_with_lifetime(&processor, data);
        assert!(result.contains("TEST"));
    }
}
