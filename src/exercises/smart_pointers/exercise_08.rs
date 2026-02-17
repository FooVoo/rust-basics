//! Exercise 08: Box vs Stack - Compare heap and stack allocation
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand when to use Box vs stack allocation
//! - Learn about large stack allocations
//! - Work with Box for large data structures

const LARGE_SIZE: usize = 10000;

#[derive(Clone)]
pub struct LargeStruct {
    pub data: [u8; LARGE_SIZE],
}

impl LargeStruct {
    pub fn new() -> Self  {
        todo!("Implement new")
    }

    pub fn sum(&self) -> u64  {
        todo!("Implement sum")
    }
}

/// Create a boxed large struct to avoid stack overflow.
pub fn create_on_heap() -> Box<LargeStruct>  {
    todo!("Create a boxed large struct to avoid stack overflow.")
}

/// Create multiple large structs on heap.
pub fn create_multiple_on_heap(count: usize) -> Vec<Box<LargeStruct>>  {
    todo!("Create multiple large structs on heap.")
}

/// Set values in a boxed large struct.
pub fn set_values(mut boxed: Box<LargeStruct>, value: u8) -> Box<LargeStruct>  {
    todo!("Set values in a boxed large struct.")
}

/// Compare sizes.
pub fn size_of_box_vs_value() -> (usize, usize)  {
    todo!("Compare sizes.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_on_heap() {
        let large = create_on_heap();
        assert_eq!(large.data.len(), LARGE_SIZE);
        assert_eq!(large.sum(), 0);
    }

    #[test]
    fn test_set_values() {
        let large = create_on_heap();
        let modified = set_values(large, 5);
        assert_eq!(modified.sum(), (5 * LARGE_SIZE) as u64);
    }

    #[test]
    fn test_multiple_on_heap() {
        let structs = create_multiple_on_heap(10);
        assert_eq!(structs.len(), 10);
        for s in &structs {
            assert_eq!(s.sum(), 0);
        }
    }

    #[test]
    fn test_box_size() {
        let (box_size, value_size) = size_of_box_vs_value();
        // Box is just a pointer, much smaller than the actual value
        assert!(box_size < value_size);
        // Box size should be the size of a pointer (8 bytes on 64-bit)
        assert_eq!(box_size, std::mem::size_of::<usize>());
    }

    #[test]
    fn test_box_is_pointer_sized() {
        // This ensures Box doesn't add overhead beyond a pointer
        assert_eq!(
            std::mem::size_of::<Box<LargeStruct>>(),
            std::mem::size_of::<*const LargeStruct>()
        );
    }
}
