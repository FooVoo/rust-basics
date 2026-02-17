//! Exercise 09: Flat Map - Flatten and transform
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use flat_map() to flatten nested structures
//! - Understand the difference between map and flat_map
//! - Work with nested collections

/// Flatten a vector of vectors.
pub fn flatten_vectors(nested: &[Vec<i32>]) -> Vec<i32> {
    todo!("Implement flatten_vectors")
}

/// Split strings and collect all words.
pub fn split_and_collect(strings: &[&str]) -> Vec<String> {
    todo!("Implement split_and_collect")
}

/// Get all characters from strings.
pub fn all_chars(strings: &[&str]) -> Vec<char> {
    todo!("Implement all_chars")
}

/// Generate pairs for each number with a list of multipliers.
pub fn generate_pairs(numbers: &[i32], multipliers: &[i32]) -> Vec<(i32, i32)> {
    todo!("Implement generate_pairs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_vectors() {
        assert_eq!(
            flatten_vectors(&[vec![1, 2], vec![3, 4], vec![5]]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(flatten_vectors(&[]), vec![]);
        assert_eq!(flatten_vectors(&[vec![], vec![1], vec![]]), vec![1]);
    }

    #[test]
    fn test_split_and_collect() {
        assert_eq!(
            split_and_collect(&["hello world", "rust programming"]),
            vec!["hello", "world", "rust", "programming"]
        );
        assert_eq!(split_and_collect(&[]), Vec::<String>::new());
        assert_eq!(split_and_collect(&["  a  b  "]), vec!["a", "b"]);
    }

    #[test]
    fn test_all_chars() {
        assert_eq!(all_chars(&["ab", "cd"]), vec!['a', 'b', 'c', 'd']);
        assert_eq!(all_chars(&[]), vec![]);
        assert_eq!(all_chars(&["", "a", ""]), vec!['a']);
    }

    #[test]
    fn test_generate_pairs() {
        assert_eq!(
            generate_pairs(&[1, 2], &[10, 100]),
            vec![(1, 10), (1, 100), (2, 20), (2, 200)]
        );
        assert_eq!(generate_pairs(&[], &[1, 2]), vec![]);
        assert_eq!(generate_pairs(&[1], &[]), vec![]);
    }
}
