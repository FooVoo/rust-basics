//! Exercise 20: Cache Pattern - Borrowing with interior state
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Design APIs with borrowing in mind
//! - Work with owned and borrowed data
//! - Implement efficient data access patterns

pub struct DataStore {
    data: Vec<String>,
}

impl DataStore {
    pub fn new() -> Self  {
        todo!("Implement new")
    }
    
    pub fn add(&mut self, item: String)  {
        todo!("Implement add")
    }
    
    pub fn get(&self, index: usize) -> Option<&String>  {
        todo!("Implement get")
    }
    
    pub fn find(&self, query: &str) -> Option<&String>  {
        todo!("Implement find")
    }
    
    pub fn len(&self) -> usize  {
        todo!("Implement len")
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &String>  {
        todo!("Implement iter")
    }
}

/// Filter strings from store matching predicate.
pub fn filter_store<'a, F>(store: &'a DataStore, predicate: F) -> Vec<&'a String>
where
    F: Fn(&String) -> bool,
 {
    todo!("Filter strings from store matching predicate.")
}

/// Find longest string in store.
pub fn longest_in_store(store: &DataStore) -> Option<&String>  {
    todo!("Find longest string in store.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_store() -> DataStore {
        let mut store = DataStore::new();
        store.add(String::from("apple"));
        store.add(String::from("banana"));
        store.add(String::from("cherry"));
        store
    }

    #[test]
    fn test_store_new() {
        let store = DataStore::new();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_store_add_and_get() {
        let mut store = DataStore::new();
        store.add(String::from("test"));
        assert_eq!(store.get(0), Some(&String::from("test")));
        assert_eq!(store.get(1), None);
    }

    #[test]
    fn test_store_find() {
        let store = create_test_store();
        assert_eq!(store.find("ban"), Some(&String::from("banana")));
        assert_eq!(store.find("grape"), None);
    }

    #[test]
    fn test_filter_store() {
        let store = create_test_store();
        let filtered = filter_store(&store, |s| s.len() > 5);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_longest_in_store() {
        let store = create_test_store();
        let longest = longest_in_store(&store);
        assert_eq!(longest, Some(&String::from("cherry")));
    }

    #[test]
    fn test_iter() {
        let store = create_test_store();
        let count = store.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_multiple_borrows() {
        let store = create_test_store();
        let first = store.get(0);
        let found = store.find("cherry");
        let longest = longest_in_store(&store);
        
        assert!(first.is_some());
        assert!(found.is_some());
        assert!(longest.is_some());
    }
}
