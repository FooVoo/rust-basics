//! Exercise 17: Transaction Handling - All-or-nothing operations
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement transactional operations
//! - Roll back on errors
//! - Maintain consistency

#[derive(Debug, PartialEq)]
pub struct Account {
    pub id: u32,
    pub balance: i64,
}

impl Account {
    pub fn new(id: u32, balance: i64) -> Self {
        Account { id, balance }
    }
}

#[derive(Debug, PartialEq)]
pub enum TransactionError {
    InsufficientFunds,
    AccountNotFound,
    NegativeAmount,
    SameAccount,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionError::InsufficientFunds => write!(f, "Insufficient funds"),
            TransactionError::AccountNotFound => write!(f, "Account not found"),
            TransactionError::NegativeAmount => write!(f, "Amount must be positive"),
            TransactionError::SameAccount => write!(f, "Cannot transfer to same account"),
        }
    }
}

impl std::error::Error for TransactionError {}

/// Transfer money between accounts.
/// Returns Ok with updated balances or Err without modifying accounts.
pub fn transfer(
    from: &mut Account,
    to: &mut Account,
    amount: i64,
) -> Result<(), TransactionError> {
    if from.id == to.id {
        return Err(TransactionError::SameAccount);
    }
    
    if amount < 0 {
        return Err(TransactionError::NegativeAmount);
    }
    
    if from.balance < amount {
        return Err(TransactionError::InsufficientFunds);
    }
    
    from.balance -= amount;
    to.balance += amount;
    Ok(())
}

/// Withdraw money from an account.
pub fn withdraw(account: &mut Account, amount: i64) -> Result<(), TransactionError> {
    if amount < 0 {
        return Err(TransactionError::NegativeAmount);
    }
    
    if account.balance < amount {
        return Err(TransactionError::InsufficientFunds);
    }
    
    account.balance -= amount;
    Ok(())
}

/// Deposit money to an account.
pub fn deposit(account: &mut Account, amount: i64) -> Result<(), TransactionError> {
    if amount < 0 {
        return Err(TransactionError::NegativeAmount);
    }
    
    account.balance += amount;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_success() {
        let mut from = Account::new(1, 1000);
        let mut to = Account::new(2, 500);
        
        assert_eq!(transfer(&mut from, &mut to, 300), Ok(()));
        assert_eq!(from.balance, 700);
        assert_eq!(to.balance, 800);
    }

    #[test]
    fn test_transfer_insufficient_funds() {
        let mut from = Account::new(1, 100);
        let mut to = Account::new(2, 500);
        
        let result = transfer(&mut from, &mut to, 200);
        assert_eq!(result, Err(TransactionError::InsufficientFunds));
        assert_eq!(from.balance, 100);
        assert_eq!(to.balance, 500);
    }

    #[test]
    fn test_transfer_same_account() {
        let mut account1 = Account::new(1, 1000);
        let mut account2 = Account::new(1, 500);
        let result = transfer(&mut account1, &mut account2, 100);
        assert_eq!(result, Err(TransactionError::SameAccount));
    }

    #[test]
    fn test_transfer_negative_amount() {
        let mut from = Account::new(1, 1000);
        let mut to = Account::new(2, 500);
        
        let result = transfer(&mut from, &mut to, -100);
        assert_eq!(result, Err(TransactionError::NegativeAmount));
        assert_eq!(from.balance, 1000);
        assert_eq!(to.balance, 500);
    }

    #[test]
    fn test_withdraw_success() {
        let mut account = Account::new(1, 1000);
        assert_eq!(withdraw(&mut account, 300), Ok(()));
        assert_eq!(account.balance, 700);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut account = Account::new(1, 100);
        let result = withdraw(&mut account, 200);
        assert_eq!(result, Err(TransactionError::InsufficientFunds));
        assert_eq!(account.balance, 100);
    }

    #[test]
    fn test_deposit_success() {
        let mut account = Account::new(1, 1000);
        assert_eq!(deposit(&mut account, 500), Ok(()));
        assert_eq!(account.balance, 1500);
    }

    #[test]
    fn test_deposit_negative_amount() {
        let mut account = Account::new(1, 1000);
        let result = deposit(&mut account, -100);
        assert_eq!(result, Err(TransactionError::NegativeAmount));
        assert_eq!(account.balance, 1000);
    }
}
