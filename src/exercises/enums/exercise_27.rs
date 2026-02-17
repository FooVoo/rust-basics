//! Exercise 27: Generic State Machine - Type-Safe State Transitions
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create generic enum state machines
//! - Use phantom types for compile-time state validation
//! - Implement complex generic patterns

use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Locked;

#[derive(Debug, PartialEq)]
pub struct Unlocked;

#[derive(Debug)]
pub struct Door<State> {
    code: String,
    _state: PhantomData<State>,
}

impl Door<Locked> {
    /// Creates a new locked door
    pub fn new(code: String) -> Self {
        Door {
            code,
            _state: PhantomData,
        }
    }

    /// Unlocks the door with the correct code
    pub fn unlock(self, code: &str) -> Result<Door<Unlocked>, String> {
        if self.code == code {
            Ok(Door {
                code: self.code,
                _state: PhantomData,
            })
        } else {
            Err("Incorrect code".to_string())
        }
    }
}

impl Door<Unlocked> {
    /// Locks the door
    pub fn lock(self) -> Door<Locked> {
        Door {
            code: self.code,
            _state: PhantomData,
        }
    }

    /// Opens the door (only possible when unlocked)
    pub fn open(&self) -> String {
        "Door is now open".to_string()
    }
}

/// Enum-based alternative for runtime state management
#[derive(Debug, PartialEq, Clone)]
pub enum DoorState {
    Locked { code: String },
    Unlocked { code: String },
}

impl DoorState {
    /// Creates a new locked door
    pub fn new(code: String) -> Self {
        DoorState::Locked { code }
    }

    /// Attempts to unlock the door
    pub fn unlock(&mut self, input_code: &str) -> Result<(), String> {
        match self {
            DoorState::Locked { code } => {
                if code == input_code {
                    let code = code.clone();
                    *self = DoorState::Unlocked { code };
                    Ok(())
                } else {
                    Err("Incorrect code".to_string())
                }
            }
            DoorState::Unlocked { .. } => Err("Already unlocked".to_string()),
        }
    }

    /// Locks the door
    pub fn lock(&mut self) -> Result<(), String> {
        match self {
            DoorState::Unlocked { code } => {
                let code = code.clone();
                *self = DoorState::Locked { code };
                Ok(())
            }
            DoorState::Locked { .. } => Err("Already locked".to_string()),
        }
    }

    /// Checks if door is locked
    pub fn is_locked(&self) -> bool {
        matches!(self, DoorState::Locked { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_state_door() {
        let door = Door::<Locked>::new("1234".to_string());
        let door = door.unlock("1234").unwrap();
        assert_eq!(door.open(), "Door is now open");
        let _door = door.lock();
    }

    #[test]
    fn test_type_state_wrong_code() {
        let door = Door::<Locked>::new("1234".to_string());
        assert!(door.unlock("0000").is_err());
    }

    #[test]
    fn test_enum_state_door() {
        let mut door = DoorState::new("1234".to_string());
        assert!(door.is_locked());
        assert!(door.unlock("1234").is_ok());
        assert!(!door.is_locked());
        assert!(door.lock().is_ok());
        assert!(door.is_locked());
    }

    #[test]
    fn test_enum_state_wrong_code() {
        let mut door = DoorState::new("1234".to_string());
        assert!(door.unlock("0000").is_err());
        assert!(door.is_locked());
    }
}
