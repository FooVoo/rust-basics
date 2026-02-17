//! Exercise 24: Associated Types with Generics - Combine associated types with generic parameters
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use associated types with generics
//! - Understand the difference between generic parameters and associated types
//! - Implement complex trait relationships

/// A trait for types that can be converted to another type.
pub trait Converter {
    type Input;
    type Output;
    
    fn convert(&self, input: Self::Input) -> Self::Output;
}

/// A generic converter that uses a function.
pub struct FunctionConverter<I, O, F>
where
    F: Fn(I) -> O,
{
    func: F,
    _phantom: std::marker::PhantomData<(I, O)>,
}

impl<I, O, F> FunctionConverter<I, O, F>
where
    F: Fn(I) -> O,
{
    /// Creates a new function converter.
    pub fn new(func: F) -> Self {
        todo!("Implement new")
    }
}

impl<I, O, F> Converter for FunctionConverter<I, O, F>
where
    F: Fn(I) -> O,
{
    type Input = I;
    type Output = O;

    fn convert(&self, input: Self::Input) -> Self::Output {
        todo!("Implement convert")
    }
}

/// A trait for containers that can be iterated.
pub trait Container {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
    
    fn iter(&self) -> Self::Iter;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        todo!("Implement is_empty")
    }
}

/// A generic wrapper that implements Container.
pub struct Wrapper<T> {
    items: Vec<T>,
}

impl<T> Wrapper<T> {
    /// Creates a new wrapper.
    pub fn new(items: Vec<T>) -> Self {
        todo!("Implement new")
    }
}

impl<T: Clone> Container for Wrapper<T> {
    type Item = T;
    type Iter = std::vec::IntoIter<T>;

    fn iter(&self) -> Self::Iter {
        todo!("Implement iter")
    }

    fn len(&self) -> usize {
        todo!("Implement len")
    }
}

/// A trait for types that can be mapped to another type.
pub trait Mappable {
    type Item;
    
    fn map<U, F>(self, f: F) -> MappedResult<U>
    where
        F: FnOnce(Self::Item) -> U;
}

/// Result of a map operation.
pub struct MappedResult<T> {
    value: Option<T>,
}

impl<T> MappedResult<T> {
    /// Creates a new MappedResult.
    pub fn new(value: Option<T>) -> Self {
        todo!("Implement new")
    }

    /// Unwraps the value.
    pub fn unwrap(self) -> T {
        todo!("Implement unwrap")
    }

    /// Gets the value or a default.
    pub fn unwrap_or(self, default: T) -> T {
        todo!("Implement unwrap_or")
    }
}

impl<T> Mappable for Option<T> {
    type Item = T;

    fn map<U, F>(self, f: F) -> MappedResult<U>
    where
        F: FnOnce(Self::Item) -> U,
     {
        todo!("Implement map")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_converter() {
        let converter = FunctionConverter::new(|x: i32| x * 2);
        assert_eq!(converter.convert(5), 10);
    }

    #[test]
    fn test_function_converter_string() {
        let converter = FunctionConverter::new(|x: i32| x.to_string());
        assert_eq!(converter.convert(42), "42");
    }

    #[test]
    fn test_wrapper_new() {
        let wrapper = Wrapper::new(vec![1, 2, 3]);
        assert_eq!(wrapper.len(), 3);
    }

    #[test]
    fn test_wrapper_iter() {
        let wrapper = Wrapper::new(vec![1, 2, 3]);
        let sum: i32 = wrapper.iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_wrapper_is_empty() {
        let empty: Wrapper<i32> = Wrapper::new(vec![]);
        assert!(empty.is_empty());
        
        let not_empty = Wrapper::new(vec![1]);
        assert!(!not_empty.is_empty());
    }

    #[test]
    fn test_mappable_some() {
        let opt = Some(5);
        let result = opt.map(|x| x * 2);
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_mappable_none() {
        let opt: Option<i32> = None;
        let result = opt.map(|x| x * 2);
        assert_eq!(result.unwrap_or(0), 0);
    }

    #[test]
    fn test_converter_chain() {
        let c1 = FunctionConverter::new(|x: i32| x * 2);
        let c2 = FunctionConverter::new(|x: i32| x + 10);
        
        let result1 = c1.convert(5);
        let result2 = c2.convert(result1);
        assert_eq!(result2, 20);
    }

    #[test]
    fn test_wrapper_multiple_iterations() {
        let wrapper = Wrapper::new(vec![1, 2, 3]);
        let sum1: i32 = wrapper.iter().sum();
        let sum2: i32 = wrapper.iter().sum();
        assert_eq!(sum1, sum2);
    }
}
