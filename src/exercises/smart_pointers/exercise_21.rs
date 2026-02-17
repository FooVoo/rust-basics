//! Exercise 21: Cow Basics - Clone-on-write smart pointer
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand Cow<T> for efficient clone-on-write
//! - Learn when Cow borrows vs owns
//! - Optimize memory with Cow

use std::borrow::Cow;

/// Return a Cow that borrows if no modification needed.
pub fn maybe_uppercase(s: &str) -> Cow<str> {
    todo!("Implement maybe_uppercase")
}

/// Process a slice, modifying only if needed.
pub fn ensure_positive(nums: &[i32]) -> Cow<[i32]> {
    todo!("Implement ensure_positive")
}

/// Append to string only if it doesn't end with suffix.
pub fn ensure_suffix<'a>(s: &'a str, suffix: &str) -> Cow<'a, str> {
    todo!("Implement ensure_suffix")
}

/// Count how many Cow instances actually own their data.
pub fn count_owned(cows: &[Cow<str>]) -> usize {
    todo!("Implement count_owned")
}

/// Convert Cow to owned String.
pub fn cow_to_owned(cow: Cow<str>) -> String {
    todo!("Implement cow_to_owned")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_uppercase_borrow() {
        let s = "HELLO";
        let result = maybe_uppercase(s);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_maybe_uppercase_owned() {
        let s = "Hello";
        let result = maybe_uppercase(s);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_ensure_positive_borrow() {
        let nums = [1, 2, 3, 4];
        let result = ensure_positive(&nums);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_ensure_positive_owned() {
        let nums = [-1, 2, -3, 4];
        let result = ensure_positive(&nums);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(&*result, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_ensure_suffix_borrow() {
        let s = "file.txt";
        let result = ensure_suffix(s, ".txt");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn test_ensure_suffix_owned() {
        let s = "file";
        let result = ensure_suffix(s, ".txt");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn test_count_owned() {
        let cows = vec![
            Cow::Borrowed("hello"),
            Cow::Owned(String::from("world")),
            Cow::Borrowed("rust"),
            Cow::Owned(String::from("cow")),
        ];
        assert_eq!(count_owned(&cows), 2);
    }

    #[test]
    fn test_cow_to_owned() {
        let borrowed: Cow<str> = Cow::Borrowed("test");
        let owned1 = cow_to_owned(borrowed);
        assert_eq!(owned1, "test");

        let owned: Cow<str> = Cow::Owned(String::from("test2"));
        let owned2 = cow_to_owned(owned);
        assert_eq!(owned2, "test2");
    }

    #[test]
    fn test_cow_efficiency() {
        // Cow avoids allocation when not needed
        let s = "ALREADY_UPPER";
        let cow = maybe_uppercase(s);
        // No allocation occurred
        assert!(matches!(cow, Cow::Borrowed(_)));
    }
}
