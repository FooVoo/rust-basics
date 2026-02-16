//! Exercise 17: Enum Iterators - Creating Enum Sequences
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create iterators over enum variants
//! - Implement iterator traits for enums
//! - Generate enum sequences

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    /// Returns all suits in order
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }

    /// Returns the next suit in circular order
    pub fn next(&self) -> Suit {
        match self {
            Suit::Hearts => Suit::Diamonds,
            Suit::Diamonds => Suit::Clubs,
            Suit::Clubs => Suit::Spades,
            Suit::Spades => Suit::Hearts,
        }
    }

    /// Returns true if the suit is red (Hearts or Diamonds)
    pub fn is_red(&self) -> bool {
        matches!(self, Suit::Hearts | Suit::Diamonds)
    }
}

/// Generates a vector of n suits starting from the given suit
pub fn generate_suit_sequence(start: Suit, n: usize) -> Vec<Suit> {
    let mut result = Vec::with_capacity(n);
    let mut current = start;
    for _ in 0..n {
        result.push(current);
        current = current.next();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let all = Suit::all();
        assert_eq!(all.len(), 4);
        assert_eq!(all[0], Suit::Hearts);
        assert_eq!(all[3], Suit::Spades);
    }

    #[test]
    fn test_next() {
        assert_eq!(Suit::Hearts.next(), Suit::Diamonds);
        assert_eq!(Suit::Diamonds.next(), Suit::Clubs);
        assert_eq!(Suit::Clubs.next(), Suit::Spades);
        assert_eq!(Suit::Spades.next(), Suit::Hearts);
    }

    #[test]
    fn test_is_red() {
        assert!(Suit::Hearts.is_red());
        assert!(Suit::Diamonds.is_red());
        assert!(!Suit::Clubs.is_red());
        assert!(!Suit::Spades.is_red());
    }

    #[test]
    fn test_generate_suit_sequence() {
        let seq = generate_suit_sequence(Suit::Hearts, 5);
        assert_eq!(
            seq,
            vec![
                Suit::Hearts,
                Suit::Diamonds,
                Suit::Clubs,
                Suit::Spades,
                Suit::Hearts
            ]
        );
    }
}
