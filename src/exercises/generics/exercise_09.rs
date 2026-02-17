//! Exercise 09: Generic Option Patterns - Work with Option<T>
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Master Option<T> with generics
//! - Implement custom option-like types
//! - Use map, and_then patterns

/// A generic function that safely gets the first element of a slice.
pub fn first_element<T: Clone>(slice: &[T]) -> Option<T> {
    todo!("Implement first_element")
}

/// A generic function that finds an element matching a predicate.
pub fn find_by<T, F>(slice: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
 {
    todo!("A generic function that finds an element matching a predicate.")
}

/// A custom Maybe type similar to Option.
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    /// Maps a Maybe<T> to Maybe<U> by applying a function.
    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
     {
        todo!("Maps a Maybe<T> to Maybe<U> by applying a function.")
    }

    /// Returns the contained value or a default.
    pub fn unwrap_or(self, default: T) -> T {
        todo!("Implement unwrap_or")
    }

    /// Returns true if the Maybe is Just.
    pub fn is_just(&self) -> bool {
        todo!("Implement is_just")
    }

    /// Returns true if the Maybe is Nothing.
    pub fn is_nothing(&self) -> bool {
        todo!("Implement is_nothing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_element_present() {
        let numbers = vec![1, 2, 3];
        assert_eq!(first_element(&numbers), Some(1));
    }

    #[test]
    fn test_first_element_empty() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(first_element(&numbers), None);
    }

    #[test]
    fn test_find_by_found() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = find_by(&numbers, |&x| x > 3);
        assert_eq!(result, Some(&4));
    }

    #[test]
    fn test_find_by_not_found() {
        let numbers = vec![1, 2, 3];
        let result = find_by(&numbers, |&x| x > 10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_maybe_just() {
        let m = Maybe::Just(42);
        assert!(m.is_just());
        assert!(!m.is_nothing());
    }

    #[test]
    fn test_maybe_nothing() {
        let m: Maybe<i32> = Maybe::Nothing;
        assert!(m.is_nothing());
        assert!(!m.is_just());
    }

    #[test]
    fn test_maybe_map() {
        let m = Maybe::Just(5);
        let result = m.map(|x| x * 2);
        assert_eq!(result.unwrap_or(0), 10);
    }

    #[test]
    fn test_maybe_map_nothing() {
        let m: Maybe<i32> = Maybe::Nothing;
        let result = m.map(|x| x * 2);
        assert!(result.is_nothing());
    }

    #[test]
    fn test_maybe_unwrap_or() {
        let just = Maybe::Just(42);
        let nothing: Maybe<i32> = Maybe::Nothing;
        assert_eq!(just.unwrap_or(0), 42);
        assert_eq!(nothing.unwrap_or(0), 0);
    }
}
