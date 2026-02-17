//! Exercise 15: Generic State Machine - Implement a generic state machine
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create generic state machines
//! - Use enums with generic parameters
//! - Implement state transitions

/// A generic state machine with three states.
pub enum State<T, U, V> {
    Initial(T),
    Processing(U),
    Complete(V),
}

impl<T, U, V> State<T, U, V> {
    /// Creates a new state machine in the Initial state.
    pub fn new(value: T) -> Self  {
        todo!("Create a new state machine in the Initial state.")
    }

    /// Checks if the state is Initial.
    pub fn is_initial(&self) -> bool  {
        todo!("Check if the state is Initial.")
    }

    /// Checks if the state is Processing.
    pub fn is_processing(&self) -> bool  {
        todo!("Check if the state is Processing.")
    }

    /// Checks if the state is Complete.
    pub fn is_complete(&self) -> bool  {
        todo!("Check if the state is Complete.")
    }

    /// Transitions from Initial to Processing.
    pub fn start_processing<F>(self, f: F) -> Result<State<T, U, V>, &'static str>
    where
        F: FnOnce(T) -> U,
     {
        todo!("Transitions from Initial to Processing.")
    }

    /// Transitions from Processing to Complete.
    pub fn complete<F>(self, f: F) -> Result<State<T, U, V>, &'static str>
    where
        F: FnOnce(U) -> V,
     {
        todo!("Transitions from Processing to Complete.")
    }
}

impl<T: Clone, U: Clone, V: Clone> State<T, U, V> {
    /// Gets a clone of the current state value.
    pub fn get_value(&self) -> StateValue<T, U, V>  {
        todo!("Get a clone of the current state value.")
    }
}

/// Represents the value in a state.
pub enum StateValue<T, U, V> {
    Initial(T),
    Processing(U),
    Complete(V),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state: State<i32, String, bool> = State::new(42);
        assert!(state.is_initial());
        assert!(!state.is_processing());
        assert!(!state.is_complete());
    }

    #[test]
    fn test_transition_to_processing() {
        let state: State<i32, String, bool> = State::new(42);
        let state = state.start_processing(|x| format!("Processing {}", x)).unwrap();
        assert!(state.is_processing());
    }

    #[test]
    fn test_transition_to_complete() {
        let state: State<i32, String, bool> = State::new(42);
        let state = state.start_processing(|x| format!("Processing {}", x)).unwrap();
        let state = state.complete(|_| true).unwrap();
        assert!(state.is_complete());
    }

    #[test]
    fn test_invalid_transition_from_complete() {
        let state: State<i32, String, bool> = State::new(42);
        let state = state.start_processing(|x| x.to_string()).unwrap();
        let state = state.complete(|_| true).unwrap();
        let result = state.start_processing(|x| x.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_complete_from_initial() {
        let state: State<i32, String, bool> = State::new(42);
        let result = state.complete(|s: String| !s.is_empty());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_value_initial() {
        let state: State<i32, String, bool> = State::new(42);
        let value = state.get_value();
        assert!(matches!(value, StateValue::Initial(42)));
    }

    #[test]
    fn test_full_workflow() {
        let state: State<i32, i32, i32> = State::new(10);
        let state = state.start_processing(|x| x * 2).unwrap();
        let state = state.complete(|x| x + 5).unwrap();
        assert!(state.is_complete());
    }
}
