//! Exercise 13: Nested Enums - Enums Containing Enums
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Work with nested enum structures
//! - Pattern match on nested enums
//! - Build complex data representations

#[derive(Debug, PartialEq, Clone)]
pub enum PaymentMethod {
    Cash,
    Card(CardType),
    DigitalWallet(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CardType {
    Credit,
    Debit,
}

/// Returns the payment method name
pub fn payment_name(payment: &PaymentMethod) -> String {
    match payment {
        PaymentMethod::Cash => "Cash".to_string(),
        PaymentMethod::Card(CardType::Credit) => "Credit Card".to_string(),
        PaymentMethod::Card(CardType::Debit) => "Debit Card".to_string(),
        PaymentMethod::DigitalWallet(name) => format!("Digital Wallet ({})", name),
    }
}

/// Returns true if the payment is electronic (card or digital wallet)
pub fn is_electronic(payment: &PaymentMethod) -> bool {
    !matches!(payment, PaymentMethod::Cash)
}

/// Returns processing fee percentage
pub fn processing_fee(payment: &PaymentMethod) -> f64 {
    match payment {
        PaymentMethod::Cash => 0.0,
        PaymentMethod::Card(CardType::Credit) => 2.5,
        PaymentMethod::Card(CardType::Debit) => 1.5,
        PaymentMethod::DigitalWallet(_) => 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_name() {
        assert_eq!(payment_name(&PaymentMethod::Cash), "Cash");
        assert_eq!(
            payment_name(&PaymentMethod::Card(CardType::Credit)),
            "Credit Card"
        );
        assert_eq!(
            payment_name(&PaymentMethod::Card(CardType::Debit)),
            "Debit Card"
        );
        assert_eq!(
            payment_name(&PaymentMethod::DigitalWallet("PayPal".to_string())),
            "Digital Wallet (PayPal)"
        );
    }

    #[test]
    fn test_is_electronic() {
        assert!(!is_electronic(&PaymentMethod::Cash));
        assert!(is_electronic(&PaymentMethod::Card(CardType::Credit)));
        assert!(is_electronic(&PaymentMethod::Card(CardType::Debit)));
        assert!(is_electronic(&PaymentMethod::DigitalWallet(
            "Venmo".to_string()
        )));
    }

    #[test]
    fn test_processing_fee() {
        assert_eq!(processing_fee(&PaymentMethod::Cash), 0.0);
        assert_eq!(processing_fee(&PaymentMethod::Card(CardType::Credit)), 2.5);
        assert_eq!(processing_fee(&PaymentMethod::Card(CardType::Debit)), 1.5);
        assert_eq!(
            processing_fee(&PaymentMethod::DigitalWallet("Venmo".to_string())),
            1.0
        );
    }
}
