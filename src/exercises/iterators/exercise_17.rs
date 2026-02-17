//! Exercise 17: Iterator from Ranges - Working with ranges
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use range expressions (..  and ..=)
//! - Generate sequences with step_by
//! - Create numeric sequences

/// Generate fibonacci sequence up to n terms.
pub fn fibonacci(n: usize) -> Vec<u64> {
    (0..n)
        .scan((0u64, 1u64), |state, _| {
            let current = state.0;
            *state = (state.1, state.0 + state.1);
            Some(current)
        })
        .collect()
}

/// Generate multiplication table for n.
pub fn multiplication_table(n: i32, up_to: i32) -> Vec<(i32, i32)> {
    (1..=up_to).map(|i| (i, n * i)).collect()
}

/// Sum of squares from 1 to n.
pub fn sum_of_squares(n: i32) -> i32 {
    (1..=n).map(|i| i * i).sum()
}

/// Generate even numbers in range.
pub fn even_numbers_in_range(start: i32, end: i32) -> Vec<i32> {
    (start..=end).filter(|n| n % 2 == 0).collect()
}

/// Generate powers of 2 up to n.
pub fn powers_of_two(n: usize) -> Vec<u64> {
    (0..n).map(|i| 2u64.pow(i as u32)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(7), vec![0, 1, 1, 2, 3, 5, 8]);
        assert_eq!(fibonacci(0), vec![]);
        assert_eq!(fibonacci(1), vec![0]);
        assert_eq!(fibonacci(2), vec![0, 1]);
    }

    #[test]
    fn test_multiplication_table() {
        assert_eq!(
            multiplication_table(5, 4),
            vec![(1, 5), (2, 10), (3, 15), (4, 20)]
        );
        assert_eq!(multiplication_table(3, 0), vec![]);
        assert_eq!(multiplication_table(7, 3), vec![(1, 7), (2, 14), (3, 21)]);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(3), 14); // 1 + 4 + 9
        assert_eq!(sum_of_squares(0), 0);
        assert_eq!(sum_of_squares(1), 1);
        assert_eq!(sum_of_squares(5), 55); // 1 + 4 + 9 + 16 + 25
    }

    #[test]
    fn test_even_numbers_in_range() {
        assert_eq!(even_numbers_in_range(1, 10), vec![2, 4, 6, 8, 10]);
        assert_eq!(even_numbers_in_range(1, 1), vec![]);
        assert_eq!(even_numbers_in_range(2, 2), vec![2]);
        assert_eq!(even_numbers_in_range(-3, 3), vec![-2, 0, 2]);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(powers_of_two(5), vec![1, 2, 4, 8, 16]);
        assert_eq!(powers_of_two(0), vec![]);
        assert_eq!(powers_of_two(1), vec![1]);
    }
}
