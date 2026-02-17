//! Exercise 29: Niche Value Optimization - Understanding Enum Memory Layout
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Understand enum memory layout and size optimization
//! - Learn about niche value optimization
//! - Work with discriminant values
//! - Understand Option<&T> and Option<Box<T>> optimizations

use std::mem;

#[derive(Debug, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, PartialEq)]
pub enum OptimizedEnum {
    None,
    Some(Box<u32>),
}

/// Demonstrates that Option<&T> has the same size as &T due to niche optimization
pub fn option_reference_size_demo() -> (usize, usize) {
    let ref_size = mem::size_of::<&u32>();
    let option_ref_size = mem::size_of::<Option<&u32>>();
    (ref_size, option_ref_size)
}

/// Demonstrates that Option<Box<T>> has the same size as Box<T> due to niche optimization
pub fn option_box_size_demo() -> (usize, usize) {
    let box_size = mem::size_of::<Box<u32>>();
    let option_box_size = mem::size_of::<Option<Box<u32>>>();
    (box_size, option_box_size)
}

/// Compares the size of an enum with and without data
pub fn enum_size_comparison() -> (usize, usize, usize) {
    let simple_enum_size = mem::size_of::<Status>();
    let enum_with_data = mem::size_of::<Result<u64, String>>();
    let option_enum = mem::size_of::<Option<Status>>();
    (simple_enum_size, enum_with_data, option_enum)
}

#[derive(Debug)]
#[repr(u8)]
pub enum ExplicitDiscriminant {
    First = 10,
    Second = 20,
    Third = 30,
}

impl ExplicitDiscriminant {
    /// Gets the discriminant value as u8
    pub fn discriminant(&self) -> u8 {
        match self {
            ExplicitDiscriminant::First => 10,
            ExplicitDiscriminant::Second => 20,
            ExplicitDiscriminant::Third => 30,
        }
    }
}

/// Demonstrates enum with explicit discriminants
pub fn explicit_discriminant_demo(value: ExplicitDiscriminant) -> u8 {
    value.discriminant()
}

/// Complex enum to demonstrate layout
#[derive(Debug, PartialEq)]
pub enum ComplexEnum {
    Small(u8),
    Medium(u32),
    Large(u64, u64),
    Empty,
}

/// Returns the size and alignment of ComplexEnum
pub fn complex_enum_layout() -> (usize, usize) {
    (mem::size_of::<ComplexEnum>(), mem::align_of::<ComplexEnum>())
}

/// Demonstrates fieldless enum (C-like enum)
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum FieldlessEnum {
    A,
    B,
    C,
}

/// Converts fieldless enum to u8
pub fn fieldless_to_u8(e: FieldlessEnum) -> u8 {
    e as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_reference_niche_optimization() {
        let (ref_size, option_ref_size) = option_reference_size_demo();
        // Option<&T> should be the same size as &T due to niche optimization
        assert_eq!(ref_size, option_ref_size);
    }

    #[test]
    fn test_option_box_niche_optimization() {
        let (box_size, option_box_size) = option_box_size_demo();
        // Option<Box<T>> should be the same size as Box<T> due to niche optimization
        assert_eq!(box_size, option_box_size);
    }

    #[test]
    fn test_enum_sizes() {
        let (simple, with_data, option_enum) = enum_size_comparison();
        // Simple enum should be small (typically 1 byte)
        assert!(simple <= 1);
        // Enum with data should be larger
        assert!(with_data > simple);
        // Option<SimpleEnum> should be small due to optimization
        assert!(option_enum <= 2);
    }

    #[test]
    fn test_explicit_discriminant() {
        assert_eq!(explicit_discriminant_demo(ExplicitDiscriminant::First), 10);
        assert_eq!(explicit_discriminant_demo(ExplicitDiscriminant::Second), 20);
        assert_eq!(explicit_discriminant_demo(ExplicitDiscriminant::Third), 30);
    }

    #[test]
    fn test_complex_enum_layout() {
        let (size, align) = complex_enum_layout();
        // Should have reasonable size and alignment
        assert!(size > 0);
        assert!(align > 0);
        // Size should be at least as large as the largest variant
        assert!(size >= mem::size_of::<(u64, u64)>());
    }

    #[test]
    fn test_fieldless_enum() {
        assert_eq!(fieldless_to_u8(FieldlessEnum::A), 0);
        assert_eq!(fieldless_to_u8(FieldlessEnum::B), 1);
        assert_eq!(fieldless_to_u8(FieldlessEnum::C), 2);
        // Fieldless enum should be 1 byte
        assert_eq!(mem::size_of::<FieldlessEnum>(), 1);
    }

    #[test]
    fn test_optimized_enum() {
        let none = OptimizedEnum::None;
        let some = OptimizedEnum::Some(Box::new(42));
        assert_eq!(none, OptimizedEnum::None);
        assert_eq!(some, OptimizedEnum::Some(Box::new(42)));
    }
}
