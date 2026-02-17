//! Exercise 04: Enum Methods - Implementing Methods on Enums
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Implement methods on enum types
//! - Use self in enum methods
//! - Create associated functions

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns the opposite direction
    pub fn opposite(&self) -> Direction {
        todo!("Implement opposite")
    }

    /// Returns true if the direction is horizontal (East or West)
    pub fn is_horizontal(&self) -> bool {
        todo!("Implement is_horizontal")
    }

    /// Returns true if the direction is vertical (North or South)
    pub fn is_vertical(&self) -> bool {
        todo!("Implement is_vertical")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::West.opposite(), Direction::East);
    }

    #[test]
    fn test_is_horizontal() {
        assert!(Direction::East.is_horizontal());
        assert!(Direction::West.is_horizontal());
        assert!(!Direction::North.is_horizontal());
        assert!(!Direction::South.is_horizontal());
    }

    #[test]
    fn test_is_vertical() {
        assert!(Direction::North.is_vertical());
        assert!(Direction::South.is_vertical());
        assert!(!Direction::East.is_vertical());
        assert!(!Direction::West.is_vertical());
    }
}
