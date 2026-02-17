//! Exercise 16: Iterator Inspection - Inspect and peek
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use inspect() for debugging
//! - Use peekable() to look ahead
//! - Debug iterator pipelines

/// Process numbers and count how many pass each stage.
pub fn count_stages(numbers: &[i32]) -> (usize, usize, usize)  {
    todo!("Process numbers and count how many pass each stage.")
}

/// Group consecutive equal elements.
pub fn group_consecutive_equal(numbers: Vec<i32>) -> Vec<Vec<i32>>  {
    todo!("Group consecutive equal elements.")
}

/// Take elements until a condition is met, including the matching element.
pub fn take_until_inclusive<I>(iter: I, predicate: impl Fn(&i32) -> bool) -> Vec<i32>
where
    I: IntoIterator<Item = i32>,
 {
    todo!("Take elements until a condition is met, including the matching element.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_stages() {
        let (total, filtered, final_count) = count_stages(&[1, -2, 3, -4, 5]);
        assert_eq!(total, 5);
        assert_eq!(filtered, 3);
        assert_eq!(final_count, 3);
        
        let (total, filtered, final_count) = count_stages(&[-1, -2, -3]);
        assert_eq!(total, 3);
        assert_eq!(filtered, 0);
        assert_eq!(final_count, 0);
    }

    #[test]
    fn test_group_consecutive_equal() {
        assert_eq!(
            group_consecutive_equal(vec![1, 1, 2, 2, 2, 3, 1]),
            vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1]]
        );
        assert_eq!(group_consecutive_equal(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(group_consecutive_equal(vec![1]), vec![vec![1]]);
        assert_eq!(
            group_consecutive_equal(vec![1, 2, 3]),
            vec![vec![1], vec![2], vec![3]]
        );
    }

    #[test]
    fn test_take_until_inclusive() {
        assert_eq!(
            take_until_inclusive(vec![1, 2, 3, 4, 5], |&n| n == 3),
            vec![1, 2, 3]
        );
        assert_eq!(
            take_until_inclusive(vec![1, 2, 3], |&n| n > 10),
            vec![1, 2, 3]
        );
        assert_eq!(take_until_inclusive(vec![], |&n| n == 1), vec![]);
    }
}
