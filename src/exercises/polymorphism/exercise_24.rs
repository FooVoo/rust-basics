//! Exercise 24: Blanket Implementations - Implement traits for all types matching criteria
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create blanket implementations
//! - Understand coherence and orphan rules
//! - Implement traits conditionally for multiple types

use std::fmt::Display;

pub trait Stringify {
    fn stringify(&self) -> String;
}

// Blanket implementation for all types implementing Display
impl<T: Display> Stringify for T {
    fn stringify(&self) -> String  {
        todo!("Implement stringify")
    }
}

pub trait Double {
    fn double(&self) -> Self;
}

// Blanket implementation for numeric types
impl<T> Double for T
where
    T: std::ops::Add<Output = T> + Copy,
{
    fn double(&self) -> Self  {
        todo!("Implement double")
    }
}

pub trait Collector<T> {
    fn collect_items(&self) -> Vec<T>;
}

// Blanket implementation for any slice
impl<T: Clone> Collector<T> for [T] {
    fn collect_items(&self) -> Vec<T>  {
        todo!("Implement collect_items")
    }
}

pub trait AsVec {
    type Item;
    fn as_vec(&self) -> Vec<Self::Item>;
}

// Blanket implementation for iterators
impl<I, T> AsVec for I
where
    I: Iterator<Item = T> + Clone,
    T: Clone,
{
    type Item = T;
    
    fn as_vec(&self) -> Vec<Self::Item>  {
        todo!("Implement as_vec")
    }
}

pub trait Mapper<T> {
    fn map_all<F, U>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U;
}

// Blanket implementation for Vec<T>
impl<T> Mapper<T> for Vec<T> {
    fn map_all<F, U>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
     {
        todo!("Implement map_all")
    }
}

pub trait Filterable<T> {
    fn filter_by<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
        T: Clone;
}

// Blanket implementation for anything that can be turned into an iterator
impl<T, C> Filterable<T> for C
where
    C: AsRef<[T]>,
    T: Clone,
{
    fn filter_by<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
        T: Clone,
     {
        todo!("Implement filter_by")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify_i32() {
        let num = 42;
        assert_eq!(num.stringify(), "String: 42");
    }

    #[test]
    fn test_stringify_string() {
        let text = "hello".to_string();
        assert_eq!(text.stringify(), "String: hello");
    }

    #[test]
    fn test_stringify_bool() {
        assert_eq!(true.stringify(), "String: true");
        assert_eq!(false.stringify(), "String: false");
    }

    #[test]
    fn test_double_i32() {
        assert_eq!(5.double(), 10);
        assert_eq!((-3).double(), -6);
    }

    #[test]
    fn test_double_f64() {
        assert_eq!(2.5.double(), 5.0);
        assert_eq!(1.0.double(), 2.0);
    }

    #[test]
    fn test_collector_slice() {
        let arr = [1, 2, 3, 4];
        let collected = arr.collect_items();
        assert_eq!(collected, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_collector_string_slice() {
        let strings = ["a", "b", "c"];
        let collected = strings.collect_items();
        assert_eq!(collected, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_mapper_vec() {
        let numbers = vec![1, 2, 3, 4];
        let doubled = numbers.map_all(|x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_mapper_strings() {
        let words = vec!["hello", "world"];
        let lengths = words.map_all(|s| s.len());
        assert_eq!(lengths, vec![5, 5]);
    }

    #[test]
    fn test_filterable_vec() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let evens = numbers.filter_by(|x| *x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn test_filterable_strings() {
        let words = vec!["apple", "banana", "apricot", "cherry"];
        let a_words = words.filter_by(|s| s.starts_with('a'));
        assert_eq!(a_words, vec!["apple", "apricot"]);
    }

    #[test]
    fn test_multiple_blanket_impls() {
        let num = 10;
        
        // Gets Stringify from blanket impl
        assert!(num.stringify().contains("10"));
        
        // Gets Double from blanket impl
        assert_eq!(num.double(), 20);
    }

    #[test]
    fn test_blanket_impl_composition() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        // Use Mapper
        let doubled = numbers.map_all(|x| x * 2);
        
        // Use Filterable
        let evens = numbers.filter_by(|x| *x % 2 == 0);
        
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_different_types_same_trait() {
        // Different types, same blanket implementation
        assert_eq!(5.double(), 10);
        assert_eq!(2.5.double(), 5.0);
        assert_eq!('A'.stringify(), "String: A");
        assert_eq!(100_u64.stringify(), "String: 100");
    }

    #[test]
    fn test_chain_blanket_operations() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result = numbers
            .filter_by(|x| *x % 2 == 0)
            .into_iter()
            .map(|x| x.double())
            .collect::<Vec<_>>();
        
        assert_eq!(result, vec![4, 8, 12]);
    }
}
