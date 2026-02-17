//! Exercise 29: Understanding VTables and Dispatch - Deep dive into polymorphism internals
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Understand vtable structure and function pointers
//! - Compare static vs dynamic dispatch performance characteristics
//! - Understand fat pointers and trait object representation

use std::fmt::Debug;

pub trait Operation {
    fn execute(&self) -> i32;
    fn name(&self) -> &'static str;
}

pub struct Add {
    pub a: i32,
    pub b: i32,
}

impl Operation for Add {
    fn execute(&self) -> i32  {
        todo!("Implement execute")
    }
    
    fn name(&self) -> &'static str  {
        todo!("Implement name")
    }
}

pub struct Multiply {
    pub a: i32,
    pub b: i32,
}

impl Operation for Multiply {
    fn execute(&self) -> i32  {
        todo!("Implement execute")
    }
    
    fn name(&self) -> &'static str  {
        todo!("Implement name")
    }
}

pub struct Subtract {
    pub a: i32,
    pub b: i32,
}

impl Operation for Subtract {
    fn execute(&self) -> i32  {
        todo!("Implement execute")
    }
    
    fn name(&self) -> &'static str  {
        todo!("Implement name")
    }
}

/// Static dispatch - compiler knows exact type, can inline
pub fn execute_static<T: Operation>(op: &T) -> (i32, &'static str)  {
    todo!("Static dispatch - compiler knows exact type, can inline")
}

/// Dynamic dispatch - runtime lookup through vtable
pub fn execute_dynamic(op: &dyn Operation) -> (i32, &'static str)  {
    todo!("Dynamic dispatch - runtime lookup through vtable")
}

/// Demonstrate fat pointer - trait object contains both data pointer and vtable pointer
pub fn analyze_trait_object_size() -> (usize, usize)  {
    todo!("Demonstrate fat pointer - trait object contains both data pointer and vtable pointer")
}

/// Collection using dynamic dispatch - heterogeneous
pub struct OperationChain {
    operations: Vec<Box<dyn Operation>>,
}

impl OperationChain {
    pub fn new() -> Self  {
        todo!("Collection using dynamic dispatch - heterogeneous")
    }
    
    pub fn add_operation(&mut self, op: Box<dyn Operation>)  {
        todo!("Implement add_operation")
    }
    
    pub fn execute_all(&self) -> Vec<(i32, &'static str)>  {
        todo!("Implement execute_all")
    }
    
    /// Demonstrate vtable indirection for each call
    pub fn execute_with_logging(&self) -> Vec<String>  {
        todo!("Demonstrate vtable indirection for each call")
    }
}

/// Trait with multiple methods to show vtable structure
pub trait MultiMethod: Debug {
    fn method1(&self) -> i32;
    fn method2(&self) -> i32;
    fn method3(&self) -> i32;
    fn method4(&self) -> i32;
}

#[derive(Debug)]
pub struct MultiImpl {
    value: i32,
}

impl MultiMethod for MultiImpl {
    fn method1(&self) -> i32  {
        todo!("Implement method1")
    }
    fn method2(&self) -> i32  {
        todo!("Implement method2")
    }
    fn method3(&self) -> i32  {
        todo!("Implement method3")
    }
    fn method4(&self) -> i32  {
        todo!("Implement method4")
    }
}

/// Demonstrate vtable overhead
pub fn call_all_methods_static(obj: &MultiImpl) -> Vec<i32>  {
    todo!("Demonstrate vtable overhead")
}

pub fn call_all_methods_dynamic(obj: &dyn MultiMethod) -> Vec<i32>  {
    todo!("Demonstrate vtable overhead")
}

/// Show object safety and vtable requirements
pub trait ObjectSafeVTable {
    fn process(&self, value: i32) -> i32;
    fn get_multiplier(&self) -> i32;
}

pub struct VTableImpl {
    multiplier: i32,
}

impl ObjectSafeVTable for VTableImpl {
    fn process(&self, value: i32) -> i32  {
        todo!("Implement process")
    }
    
    fn get_multiplier(&self) -> i32  {
        todo!("Implement get_multiplier")
    }
}

/// Demonstrate trait object coercion
pub fn demonstrate_coercion() -> Box<dyn Operation>  {
    todo!("Demonstrate trait object coercion")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_dispatch() {
        let add = Add { a: 5, b: 3 };
        let (result, name) = execute_static(&add);
        assert_eq!(result, 8);
        assert_eq!(name, "Add");
    }

    #[test]
    fn test_dynamic_dispatch() {
        let add = Add { a: 5, b: 3 };
        let (result, name) = execute_dynamic(&add);
        assert_eq!(result, 8);
        assert_eq!(name, "Add");
    }

    #[test]
    fn test_trait_object_size() {
        let (concrete, trait_obj) = analyze_trait_object_size();
        // Trait object reference is a fat pointer (2 words: data + vtable)
        assert_eq!(trait_obj, std::mem::size_of::<usize>() * 2);
        // Concrete type is just its data
        assert!(concrete < trait_obj);
    }

    #[test]
    fn test_operation_chain() {
        let mut chain = OperationChain::new();
        chain.add_operation(Box::new(Add { a: 1, b: 2 }));
        chain.add_operation(Box::new(Multiply { a: 3, b: 4 }));
        chain.add_operation(Box::new(Subtract { a: 10, b: 5 }));
        
        let results = chain.execute_all();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], (3, "Add"));
        assert_eq!(results[1], (12, "Multiply"));
        assert_eq!(results[2], (5, "Subtract"));
    }

    #[test]
    fn test_execute_with_logging() {
        let mut chain = OperationChain::new();
        chain.add_operation(Box::new(Add { a: 2, b: 3 }));
        chain.add_operation(Box::new(Multiply { a: 4, b: 5 }));
        
        let logs = chain.execute_with_logging();
        assert_eq!(logs.len(), 2);
        assert!(logs[0].contains("Add: 5"));
        assert!(logs[1].contains("Multiply: 20"));
    }

    #[test]
    fn test_multi_method_static() {
        let obj = MultiImpl { value: 10 };
        let results = call_all_methods_static(&obj);
        assert_eq!(results, vec![10, 20, 30, 40]);
    }

    #[test]
    fn test_multi_method_dynamic() {
        let obj = MultiImpl { value: 10 };
        let results = call_all_methods_dynamic(&obj);
        assert_eq!(results, vec![10, 20, 30, 40]);
    }

    #[test]
    fn test_vtable_impl() {
        let obj = VTableImpl { multiplier: 5 };
        let trait_obj: &dyn ObjectSafeVTable = &obj;
        
        assert_eq!(trait_obj.process(10), 50);
        assert_eq!(trait_obj.get_multiplier(), 5);
    }

    #[test]
    fn test_coercion() {
        let op = demonstrate_coercion();
        assert_eq!(op.execute(), 30);
        assert_eq!(op.name(), "Add");
    }

    #[test]
    fn test_heterogeneous_collection() {
        let ops: Vec<Box<dyn Operation>> = vec![
            Box::new(Add { a: 1, b: 1 }),
            Box::new(Multiply { a: 2, b: 3 }),
            Box::new(Subtract { a: 10, b: 4 }),
            Box::new(Add { a: 5, b: 5 }),
        ];
        
        let results: Vec<_> = ops.iter().map(|op| op.execute()).collect();
        assert_eq!(results, vec![2, 6, 6, 10]);
    }

    #[test]
    fn test_vtable_indirection() {
        let add = Add { a: 10, b: 20 };
        
        // Static: direct function call, can be inlined
        let static_result = execute_static(&add);
        
        // Dynamic: goes through vtable, can't be inlined
        let dynamic_result = execute_dynamic(&add);
        
        // Results are the same, but dispatch mechanism differs
        assert_eq!(static_result, dynamic_result);
    }

    #[test]
    fn test_multiple_trait_objects() {
        let add = Add { a: 1, b: 2 };
        let mult = Multiply { a: 3, b: 4 };
        
        // Different concrete types, same trait object type
        let op1: &dyn Operation = &add;
        let op2: &dyn Operation = &mult;
        
        // Each has its own vtable
        assert_eq!(op1.execute(), 3);
        assert_eq!(op2.execute(), 12);
    }

    #[test]
    fn test_trait_object_memory_layout() {
        let add = Add { a: 5, b: 10 };
        let trait_ref: &dyn Operation = &add;
        
        // Can call methods through trait object
        assert_eq!(trait_ref.execute(), 15);
        
        // Trait object is a fat pointer
        assert_eq!(
            std::mem::size_of_val(&trait_ref),
            std::mem::size_of::<usize>() * 2
        );
    }

    #[test]
    fn test_dispatch_performance_characteristics() {
        let obj = MultiImpl { value: 100 };
        
        // Static dispatch: compiler knows exact type
        let static_results = call_all_methods_static(&obj);
        
        // Dynamic dispatch: runtime vtable lookup
        let dynamic_results = call_all_methods_dynamic(&obj);
        
        // Same results, different execution path
        assert_eq!(static_results, dynamic_results);
    }
}
