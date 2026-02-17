//! Exercise 24: State Machine Errors - Handle state transition errors
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Model state machines with error handling
//! - Validate state transitions
//! - Prevent invalid state changes

#[derive(Debug, PartialEq, Clone)]
pub enum OrderState {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, PartialEq)]
pub enum StateError {
    InvalidTransition { from: OrderState, to: OrderState },
    AlreadyInState(OrderState),
    FinalState(OrderState),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StateError::InvalidTransition { from, to } => {
                write!(f, "Cannot transition from {:?} to {:?}", from, to)
            }
            StateError::AlreadyInState(state) => {
                write!(f, "Already in state {:?}", state)
            }
            StateError::FinalState(state) => {
                write!(f, "Cannot transition from final state {:?}", state)
            }
        }
    }
}

impl std::error::Error for StateError {}

#[derive(Debug, PartialEq)]
pub struct Order {
    pub id: u32,
    pub state: OrderState,
}

impl Order {
    pub fn new(id: u32) -> Self {
        Order {
            id,
            state: OrderState::Pending,
        }
    }
    
    /// Transition to a new state if valid.
    pub fn transition(&mut self, new_state: OrderState) -> Result<(), StateError> {
        if self.state == new_state {
            return Err(StateError::AlreadyInState(new_state));
        }
        
        let valid = match (&self.state, &new_state) {
            (OrderState::Pending, OrderState::Processing) => true,
            (OrderState::Pending, OrderState::Cancelled) => true,
            (OrderState::Processing, OrderState::Shipped) => true,
            (OrderState::Processing, OrderState::Cancelled) => true,
            (OrderState::Shipped, OrderState::Delivered) => true,
            _ => false,
        };
        
        if !valid {
            if matches!(self.state, OrderState::Delivered | OrderState::Cancelled) {
                return Err(StateError::FinalState(self.state.clone()));
            }
            return Err(StateError::InvalidTransition {
                from: self.state.clone(),
                to: new_state,
            });
        }
        
        self.state = new_state;
        Ok(())
    }
    
    /// Convenience methods for common transitions
    pub fn process(&mut self) -> Result<(), StateError> {
        self.transition(OrderState::Processing)
    }
    
    pub fn ship(&mut self) -> Result<(), StateError> {
        self.transition(OrderState::Shipped)
    }
    
    pub fn deliver(&mut self) -> Result<(), StateError> {
        self.transition(OrderState::Delivered)
    }
    
    pub fn cancel(&mut self) -> Result<(), StateError> {
        self.transition(OrderState::Cancelled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order() {
        let order = Order::new(1);
        assert_eq!(order.state, OrderState::Pending);
    }

    #[test]
    fn test_valid_transition_pending_to_processing() {
        let mut order = Order::new(1);
        assert_eq!(order.process(), Ok(()));
        assert_eq!(order.state, OrderState::Processing);
    }

    #[test]
    fn test_valid_transition_full_flow() {
        let mut order = Order::new(1);
        assert!(order.process().is_ok());
        assert!(order.ship().is_ok());
        assert!(order.deliver().is_ok());
        assert_eq!(order.state, OrderState::Delivered);
    }

    #[test]
    fn test_invalid_transition_pending_to_shipped() {
        let mut order = Order::new(1);
        let result = order.ship();
        assert!(result.is_err());
        assert_eq!(order.state, OrderState::Pending);
    }

    #[test]
    fn test_invalid_transition_from_delivered() {
        let mut order = Order::new(1);
        order.state = OrderState::Delivered;
        
        let result = order.cancel();
        assert_eq!(result, Err(StateError::FinalState(OrderState::Delivered)));
    }

    #[test]
    fn test_invalid_transition_from_cancelled() {
        let mut order = Order::new(1);
        order.state = OrderState::Cancelled;
        
        let result = order.process();
        assert_eq!(result, Err(StateError::FinalState(OrderState::Cancelled)));
    }

    #[test]
    fn test_already_in_state() {
        let mut order = Order::new(1);
        order.process().unwrap();
        
        let result = order.process();
        assert_eq!(result, Err(StateError::AlreadyInState(OrderState::Processing)));
    }

    #[test]
    fn test_cancel_from_pending() {
        let mut order = Order::new(1);
        assert_eq!(order.cancel(), Ok(()));
        assert_eq!(order.state, OrderState::Cancelled);
    }

    #[test]
    fn test_cancel_from_processing() {
        let mut order = Order::new(1);
        order.process().unwrap();
        assert_eq!(order.cancel(), Ok(()));
        assert_eq!(order.state, OrderState::Cancelled);
    }

    #[test]
    fn test_cannot_cancel_shipped() {
        let mut order = Order::new(1);
        order.state = OrderState::Shipped;
        
        let result = order.cancel();
        assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
    }

    #[test]
    fn test_error_display() {
        let err = StateError::InvalidTransition {
            from: OrderState::Pending,
            to: OrderState::Shipped,
        };
        assert!(err.to_string().contains("Cannot transition"));
    }
}
