//! Exercise 18: Extension Traits - Extend existing types with new functionality
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create extension traits
//! - Add methods to existing types
//! - Use blanket implementations

pub trait StringExtensions {
    fn word_count(&self) -> usize;
    fn is_palindrome(&self) -> bool;
    fn reverse_words(&self) -> String;
}

impl StringExtensions for str {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }
    
    fn is_palindrome(&self) -> bool {
        let cleaned: String = self.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        cleaned == cleaned.chars().rev().collect::<String>()
    }
    
    fn reverse_words(&self) -> String {
        self.split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub trait VecExtensions<T> {
    fn sum_if<F>(&self, predicate: F) -> T
    where
        F: Fn(&T) -> bool,
        T: std::ops::Add<Output = T> + Default + Copy;
    
    fn all_unique(&self) -> bool
    where
        T: PartialEq;
}

impl<T> VecExtensions<T> for Vec<T> {
    fn sum_if<F>(&self, predicate: F) -> T
    where
        F: Fn(&T) -> bool,
        T: std::ops::Add<Output = T> + Default + Copy,
    {
        self.iter()
            .filter(|&&x| predicate(&x))
            .fold(T::default(), |acc, &x| acc + x)
    }
    
    fn all_unique(&self) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len() {
            for j in (i + 1)..self.len() {
                if self[i] == self[j] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!("hello world".word_count(), 2);
        assert_eq!("one two three four".word_count(), 4);
        assert_eq!("".word_count(), 0);
        assert_eq!("single".word_count(), 1);
    }

    #[test]
    fn test_is_palindrome() {
        assert!("racecar".is_palindrome());
        assert!("A man a plan a canal Panama".is_palindrome());
        assert!(!"hello".is_palindrome());
        assert!("".is_palindrome());
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!("hello world".reverse_words(), "world hello");
        assert_eq!("one two three".reverse_words(), "three two one");
        assert_eq!("single".reverse_words(), "single");
    }

    #[test]
    fn test_sum_if_evens() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let sum_evens = numbers.sum_if(|&x| x % 2 == 0);
        assert_eq!(sum_evens, 12); // 2 + 4 + 6
    }

    #[test]
    fn test_sum_if_odds() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let sum_odds = numbers.sum_if(|&x| x % 2 != 0);
        assert_eq!(sum_odds, 9); // 1 + 3 + 5
    }

    #[test]
    fn test_sum_if_greater_than() {
        let numbers = vec![1, 5, 10, 15, 20];
        let sum = numbers.sum_if(|&x| x > 10);
        assert_eq!(sum, 35); // 15 + 20
    }

    #[test]
    fn test_all_unique_true() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(numbers.all_unique());
    }

    #[test]
    fn test_all_unique_false() {
        let numbers = vec![1, 2, 3, 2, 5];
        assert!(!numbers.all_unique());
    }

    #[test]
    fn test_all_unique_empty() {
        let numbers: Vec<i32> = vec![];
        assert!(numbers.all_unique());
    }

    #[test]
    fn test_all_unique_single() {
        let numbers = vec![42];
        assert!(numbers.all_unique());
    }

    #[test]
    fn test_string_extensions_combined() {
        let text = "hello world test";
        assert_eq!(text.word_count(), 3);
        assert_eq!(text.reverse_words(), "test world hello");
        assert!(!text.is_palindrome());
    }

    #[test]
    fn test_vec_extensions_combined() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(numbers.all_unique());
        assert_eq!(numbers.sum_if(|_| true), 15);
    }
}
