//! Exercise 15: Min/Max Operations - Finding extremes
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use min(), max(), and their variants
//! - Use min_by() and max_by() with custom comparisons
//! - Find extremes based on transformations

/// Find the longest string.
pub fn longest_string(strings: &[&str]) -> Option<String> {
    strings.iter().max_by_key(|s| s.len()).map(|&s| s.to_string())
}

/// Find the string with most vowels.
pub fn most_vowels(strings: &[&str]) -> Option<String> {
    strings
        .iter()
        .max_by_key(|s| {
            s.chars()
                .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
                .count()
        })
        .map(|&s| s.to_string())
}

/// Find number closest to target.
pub fn closest_to_target(numbers: &[i32], target: i32) -> Option<i32> {
    numbers.iter().min_by_key(|&&n| (n - target).abs()).copied()
}

/// Find minimum by absolute value.
pub fn min_absolute(numbers: &[i32]) -> Option<i32> {
    numbers.iter().min_by_key(|&&n| n.abs()).copied()
}

/// Get the top n largest numbers.
pub fn top_n(numbers: &[i32], n: usize) -> Vec<i32> {
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    sorted.into_iter().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_string() {
        assert_eq!(
            longest_string(&["hi", "hello", "yo"]),
            Some("hello".to_string())
        );
        assert_eq!(longest_string(&[]), None);
        assert_eq!(longest_string(&["a"]), Some("a".to_string()));
    }

    #[test]
    fn test_most_vowels() {
        assert_eq!(
            most_vowels(&["hello", "world", "aeiou"]),
            Some("aeiou".to_string())
        );
        assert_eq!(most_vowels(&["xyz"]), Some("xyz".to_string()));
        assert_eq!(most_vowels(&[]), None);
    }

    #[test]
    fn test_closest_to_target() {
        assert_eq!(closest_to_target(&[1, 5, 10, 15], 7), Some(5));
        assert_eq!(closest_to_target(&[], 5), None);
        assert_eq!(closest_to_target(&[10], 5), Some(10));
        assert_eq!(closest_to_target(&[-5, 0, 5], 0), Some(0));
    }

    #[test]
    fn test_min_absolute() {
        assert_eq!(min_absolute(&[-5, -2, 1, 3]), Some(1));
        assert_eq!(min_absolute(&[]), None);
        assert_eq!(min_absolute(&[0, -10, 10]), Some(0));
    }

    #[test]
    fn test_top_n() {
        assert_eq!(top_n(&[3, 1, 4, 1, 5, 9], 3), vec![9, 5, 4]);
        assert_eq!(top_n(&[1, 2, 3], 5), vec![3, 2, 1]);
        assert_eq!(top_n(&[], 3), vec![]);
        assert_eq!(top_n(&[1, 2, 3], 0), vec![]);
    }
}
