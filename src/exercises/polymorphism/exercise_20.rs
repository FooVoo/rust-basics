//! Exercise 20: Trait Objects vs Generics - Compare static and dynamic dispatch
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand differences between static and dynamic dispatch
//! - Know when to use trait objects vs generics
//! - Compare performance characteristics

pub trait Processor {
    fn process(&self, input: &str) -> String;
}

pub struct UpperCaseProcessor;

impl Processor for UpperCaseProcessor {
    fn process(&self, input: &str) -> String  {
        todo!("Implement process")
    }
}

pub struct LowerCaseProcessor;

impl Processor for LowerCaseProcessor {
    fn process(&self, input: &str) -> String  {
        todo!("Implement process")
    }
}

pub struct ReverseProcessor;

impl Processor for ReverseProcessor {
    fn process(&self, input: &str) -> String  {
        todo!("Implement process")
    }
}

/// Static dispatch using generics - monomorphization
pub fn process_static<T: Processor>(processor: &T, input: &str) -> String  {
    todo!("Static dispatch using generics - monomorphization")
}

/// Dynamic dispatch using trait objects - vtable
pub fn process_dynamic(processor: &dyn Processor, input: &str) -> String  {
    todo!("Dynamic dispatch using trait objects - vtable")
}

/// Collection with static dispatch - all same type
pub fn process_all_same<T: Processor>(processors: &[T], input: &str) -> Vec<String>  {
    todo!("Collection with static dispatch - all same type")
}

/// Collection with dynamic dispatch - different types
pub fn process_all_different(processors: &[&dyn Processor], input: &str) -> Vec<String>  {
    todo!("Collection with dynamic dispatch - different types")
}

/// Factory returning trait object - runtime polymorphism
pub fn create_processor(kind: &str) -> Box<dyn Processor>  {
    todo!("Factory returning trait object - runtime polymorphism")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_processor() {
        let processor = UpperCaseProcessor;
        assert_eq!(processor.process("hello"), "HELLO");
    }

    #[test]
    fn test_lowercase_processor() {
        let processor = LowerCaseProcessor;
        assert_eq!(processor.process("WORLD"), "world");
    }

    #[test]
    fn test_reverse_processor() {
        let processor = ReverseProcessor;
        assert_eq!(processor.process("abc"), "cba");
    }

    #[test]
    fn test_process_static() {
        let processor = UpperCaseProcessor;
        let result = process_static(&processor, "test");
        assert!(result.contains("Static:"));
        assert!(result.contains("TEST"));
    }

    #[test]
    fn test_process_dynamic() {
        let processor = LowerCaseProcessor;
        let result = process_dynamic(&processor, "TEST");
        assert!(result.contains("Dynamic:"));
        assert!(result.contains("test"));
    }

    #[test]
    fn test_process_all_same() {
        let processors = vec![UpperCaseProcessor, UpperCaseProcessor];
        let results = process_all_same(&processors, "hello");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r == "HELLO"));
    }

    #[test]
    fn test_process_all_different() {
        let upper = UpperCaseProcessor;
        let lower = LowerCaseProcessor;
        let reverse = ReverseProcessor;
        
        let processors: Vec<&dyn Processor> = vec![&upper, &lower, &reverse];
        let results = process_all_different(&processors, "Test");
        
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "TEST");
        assert_eq!(results[1], "test");
        assert_eq!(results[2], "tseT");
    }

    #[test]
    fn test_create_processor_upper() {
        let processor = create_processor("upper");
        assert_eq!(processor.process("hello"), "HELLO");
    }

    #[test]
    fn test_create_processor_lower() {
        let processor = create_processor("lower");
        assert_eq!(processor.process("WORLD"), "world");
    }

    #[test]
    fn test_create_processor_reverse() {
        let processor = create_processor("reverse");
        assert_eq!(processor.process("abc"), "cba");
    }

    #[test]
    fn test_runtime_selection() {
        let kinds = vec!["upper", "lower", "reverse"];
        let processors: Vec<_> = kinds.iter().map(|k| create_processor(k)).collect();
        
        let results: Vec<_> = processors.iter()
            .map(|p| p.process("Test"))
            .collect();
        
        assert_eq!(results[0], "TEST");
        assert_eq!(results[1], "test");
        assert_eq!(results[2], "tseT");
    }

    #[test]
    fn test_static_vs_dynamic() {
        let processor = UpperCaseProcessor;
        
        // Both work, but use different dispatch mechanisms
        let static_result = process_static(&processor, "test");
        let dynamic_result = process_dynamic(&processor, "test");
        
        assert!(static_result.contains("TEST"));
        assert!(dynamic_result.contains("TEST"));
    }
}
