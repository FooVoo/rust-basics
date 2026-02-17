//! Exercise 05: Slice Borrowing - Work with string and array slices
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use slices to borrow parts of collections
//! - Work with &str (string slices)
//! - Understand slice syntax and ranges

/// Get the first word from a string.
pub fn first_word(s: &str) -> &str {
    todo!("Implement first_word")
}

/// Get a slice of a vector.
pub fn get_slice(v: &[i32], start: usize, end: usize) -> &[i32] {
    todo!("Implement get_slice")
}

/// Count words in a string.
pub fn count_words(s: &str) -> usize {
    todo!("Implement count_words")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_get_slice() {
        let v = vec![1, 2, 3, 4, 5];
        let slice = get_slice(&v, 1, 4);
        assert_eq!(slice, &[2, 3, 4]);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one"), 1);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("one two three four"), 4);
    }

    #[test]
    fn test_string_slice() {
        let s = String::from("hello world");
        let word = first_word(&s);
        assert_eq!(word, "hello");
        
        // We can also use string literals
        let word2 = first_word("rust programming");
        assert_eq!(word2, "rust");
    }

    #[test]
    fn test_slice_bounds() {
        let v = vec![10, 20, 30, 40, 50];
        assert_eq!(get_slice(&v, 0, 2), &[10, 20]);
        assert_eq!(get_slice(&v, 2, 5), &[30, 40, 50]);
    }
}
