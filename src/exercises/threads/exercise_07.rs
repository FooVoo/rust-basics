//! Exercise 07: Thread Builder
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use thread::Builder
//! - Set thread name and stack size
//! - Configure thread properties

use std::thread;

/// Create a named thread using Builder that returns its name.
pub fn create_named_thread(name: &str) -> String {
    let name = name.to_string();
    let handle = thread::Builder::new()
        .name(name.clone())
        .spawn(move || {
            thread::current().name().unwrap().to_string()
        })
        .unwrap();

    handle.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_named_thread() {
        assert_eq!(create_named_thread("worker"), "worker");
        assert_eq!(create_named_thread("thread-1"), "thread-1");
        assert_eq!(create_named_thread("test"), "test");
    }

    #[test]
    fn test_different_names() {
        let names = vec!["alpha", "beta", "gamma"];
        for name in names {
            assert_eq!(create_named_thread(name), name);
        }
    }
}
