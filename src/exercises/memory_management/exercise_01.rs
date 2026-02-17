//! Exercise 01: Basic Ownership - Transfer ownership between variables
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand ownership transfer (move semantics)
//! - Learn that values can have only one owner at a time
//! - See how ownership moves between variables

/// Takes ownership of a String and returns its length.
pub fn take_ownership(s: String) -> usize {
    todo!("Implement take_ownership")
}

/// Creates a new String and transfers ownership to the caller.
pub fn give_ownership() -> String {
    todo!("Implement give_ownership")
}

/// Takes ownership and gives it back to the caller.
pub fn take_and_give_back(s: String) -> String {
    todo!("Implement take_and_give_back")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership() {
        let s = String::from("hello");
        let len = take_ownership(s);
        assert_eq!(len, 5);
        // s is no longer valid here - ownership was moved
    }

    #[test]
    fn test_give_ownership() {
        let s = give_ownership();
        assert_eq!(s, "Hello, ownership!");
    }

    #[test]
    fn test_take_and_give_back() {
        let s1 = String::from("test");
        let s2 = take_and_give_back(s1);
        assert_eq!(s2, "test");
    }

    #[test]
    fn test_ownership_chain() {
        let s1 = give_ownership();
        let s2 = take_and_give_back(s1);
        let len = take_ownership(s2);
        assert_eq!(len, 17);
    }
}
