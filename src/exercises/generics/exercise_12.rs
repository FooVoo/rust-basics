//! Exercise 12: Generic Collections - Work with generic vectors and collections
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create generic collection utilities
//! - Work with Vec<T> generically
//! - Implement generic collection operations

/// Filters a vector based on a predicate.
pub fn filter<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
 {
    todo!("Filters a vector based on a predicate.")
}

/// Maps a vector from type T to type U.
pub fn map<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
 {
    todo!("Maps a vector from type T to type U.")
}

/// A generic collection wrapper with utility methods.
pub struct Collection<T> {
    items: Vec<T>,
}

impl<T> Collection<T> {
    /// Creates a new empty collection.
    pub fn new() -> Self  {
        todo!("Create a new empty collection.")
    }

    /// Creates a collection from a vector.
    pub fn from_vec(items: Vec<T>) -> Self  {
        todo!("Create a collection from a vector.")
    }

    /// Adds an item to the collection.
    pub fn add(&mut self, item: T)  {
        todo!("Add an item to the collection.")
    }

    /// Returns the number of items.
    pub fn len(&self) -> usize  {
        todo!("Return the number of items.")
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool  {
        todo!("Return true if the collection is empty.")
    }

    /// Converts the collection into a vector.
    pub fn into_vec(self) -> Vec<T>  {
        todo!("Convert the collection into a vector.")
    }
}

impl<T: Clone> Collection<T> {
    /// Gets a clone of the item at the given index.
    pub fn get(&self, index: usize) -> Option<T>  {
        todo!("Get a clone of the item at the given index.")
    }
}

impl<T: PartialEq> Collection<T> {
    /// Checks if the collection contains an item.
    pub fn contains(&self, item: &T) -> bool  {
        todo!("Check if the collection contains an item.")
    }

    /// Removes all occurrences of an item.
    pub fn remove_all(&mut self, item: &T)  {
        todo!("Removes all occurrences of an item.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_even() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let even = filter(numbers, |&x| x % 2 == 0);
        assert_eq!(even, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_strings() {
        let words = vec!["hello", "world", "rust", "go"];
        let long_words = filter(words, |s| s.len() > 3);
        assert_eq!(long_words, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_map_double() {
        let numbers = vec![1, 2, 3];
        let doubled = map(numbers, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_map_to_string() {
        let numbers = vec![1, 2, 3];
        let strings = map(numbers, |x| x.to_string());
        assert_eq!(strings, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_collection_new() {
        let collection: Collection<i32> = Collection::new();
        assert!(collection.is_empty());
    }

    #[test]
    fn test_collection_add() {
        let mut collection = Collection::new();
        collection.add(1);
        collection.add(2);
        assert_eq!(collection.len(), 2);
    }

    #[test]
    fn test_collection_get() {
        let collection = Collection::from_vec(vec![10, 20, 30]);
        assert_eq!(collection.get(1), Some(20));
        assert_eq!(collection.get(5), None);
    }

    #[test]
    fn test_collection_contains() {
        let collection = Collection::from_vec(vec![1, 2, 3]);
        assert!(collection.contains(&2));
        assert!(!collection.contains(&5));
    }

    #[test]
    fn test_collection_remove_all() {
        let mut collection = Collection::from_vec(vec![1, 2, 3, 2, 4]);
        collection.remove_all(&2);
        assert_eq!(collection.into_vec(), vec![1, 3, 4]);
    }
}
