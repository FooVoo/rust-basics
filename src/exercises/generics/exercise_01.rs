//! Exercise 01: Basic Generic Function - Create a simple generic function
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand generic function syntax
//! - Use type parameters
//! - Return generic values

/// A generic function that returns the first element of a pair.
pub fn first<T>(pair: (T, T)) -> T  {
    todo!("A generic function that returns the first element of a pair.")
}

/// A generic function that returns the second element of a pair.
pub fn second<T>(pair: (T, T)) -> T  {
    todo!("A generic function that returns the second element of a pair.")
}

/// A generic function that swaps elements in a tuple.
pub fn swap<T, U>(pair: (T, U)) -> (U, T)  {
    todo!("A generic function that swaps elements in a tuple.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_integers() {
        let pair = (10, 20);
        assert_eq!(first(pair), 10);
    }

    #[test]
    fn test_first_strings() {
        let pair = ("hello".to_string(), "world".to_string());
        assert_eq!(first(pair), "hello");
    }

    #[test]
    fn test_second_integers() {
        let pair = (42, 84);
        assert_eq!(second(pair), 84);
    }

    #[test]
    fn test_second_floats() {
        let pair = (3.14, 2.71);
        assert_eq!(second(pair), 2.71);
    }

    #[test]
    fn test_swap_integers() {
        let pair = (1, 2);
        assert_eq!(swap(pair), (2, 1));
    }

    #[test]
    fn test_swap_mixed_types() {
        let pair = (42, "answer".to_string());
        let swapped = swap(pair);
        assert_eq!(swapped, ("answer".to_string(), 42));
    }

    #[test]
    fn test_swap_and_first() {
        let pair = (100, 200);
        let swapped = swap(pair);
        assert_eq!(first(swapped), 200);
    }
}
