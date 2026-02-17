//! Exercise 04: Fold/Reduce - Accumulate values
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use fold() for accumulation
//! - Understand accumulator patterns
//! - Build custom aggregations

/// Product of all numbers using fold.
pub fn product(numbers: &[i32]) -> i32 {
    numbers.iter().fold(1, |acc, &n| acc * n)
}

/// Concatenate strings with a separator.
pub fn join_strings(strings: &[&str], separator: &str) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    strings[1..].iter().fold(strings[0].to_string(), |acc, &s| {
        format!("{}{}{}", acc, separator, s)
    })
}

/// Count occurrences of a character in strings.
pub fn count_char_in_strings(strings: &[&str], target: char) -> usize {
    strings
        .iter()
        .fold(0, |acc, s| acc + s.chars().filter(|&c| c == target).count())
}

/// Build a string of repeated characters.
pub fn build_repeated_string(chars: &[char], repeat: usize) -> String {
    chars
        .iter()
        .fold(String::new(), |acc, &c| {
            format!("{}{}", acc, c.to_string().repeat(repeat))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4]), 24);
        assert_eq!(product(&[]), 1);
        assert_eq!(product(&[5]), 5);
        assert_eq!(product(&[2, 0, 3]), 0);
    }

    #[test]
    fn test_join_strings() {
        assert_eq!(join_strings(&["a", "b", "c"], ", "), "a, b, c");
        assert_eq!(join_strings(&[], ", "), "");
        assert_eq!(join_strings(&["hello"], ", "), "hello");
        assert_eq!(join_strings(&["one", "two", "three"], "-"), "one-two-three");
    }

    #[test]
    fn test_count_char_in_strings() {
        assert_eq!(count_char_in_strings(&["hello", "world"], 'o'), 2);
        assert_eq!(count_char_in_strings(&[], 'a'), 0);
        assert_eq!(count_char_in_strings(&["aaa", "bbb"], 'a'), 3);
        assert_eq!(count_char_in_strings(&["test"], 'x'), 0);
    }

    #[test]
    fn test_build_repeated_string() {
        assert_eq!(build_repeated_string(&['a', 'b', 'c'], 2), "aabbcc");
        assert_eq!(build_repeated_string(&[], 3), "");
        assert_eq!(build_repeated_string(&['x'], 3), "xxx");
        assert_eq!(build_repeated_string(&['a', 'b'], 1), "ab");
    }
}
