//! Exercise 05: Box Patterns - Work with boxed closures
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Store closures in Box
//! - Work with boxed functions
//! - Understand sized vs unsized types

pub type Operation = Box<dyn Fn(i32) -> i32>;

/// Create a boxed closure that adds n to its input.
pub fn make_adder(n: i32) -> Operation {
    Box::new(move |x| x + n)
}

/// Create a boxed closure that multiplies its input by n.
pub fn make_multiplier(n: i32) -> Operation {
    Box::new(move |x| x * n)
}

/// Apply a sequence of operations to a value.
pub fn apply_operations(value: i32, ops: &[Operation]) -> i32 {
    ops.iter().fold(value, |acc, op| op(acc))
}

/// Compose two operations into one.
pub fn compose(f: Operation, g: Operation) -> Operation {
    Box::new(move |x| g(f(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_adder() {
        let add_5 = make_adder(5);
        assert_eq!(add_5(10), 15);
        assert_eq!(add_5(0), 5);
    }

    #[test]
    fn test_make_multiplier() {
        let mul_3 = make_multiplier(3);
        assert_eq!(mul_3(4), 12);
        assert_eq!(mul_3(0), 0);
    }

    #[test]
    fn test_apply_operations() {
        let ops: Vec<Operation> = vec![
            make_adder(10),
            make_multiplier(2),
            make_adder(5),
        ];
        // ((5 + 10) * 2) + 5 = 35
        assert_eq!(apply_operations(5, &ops), 35);
    }

    #[test]
    fn test_compose() {
        let add_10 = make_adder(10);
        let mul_2 = make_multiplier(2);
        let combined = compose(add_10, mul_2);
        // (5 + 10) * 2 = 30
        assert_eq!(combined(5), 30);
    }

    #[test]
    fn test_empty_operations() {
        assert_eq!(apply_operations(42, &[]), 42);
    }
}
