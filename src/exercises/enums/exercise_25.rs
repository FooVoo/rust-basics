//! Exercise 25: State Machine with Data - Advanced State Pattern
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Build complex state machines with data
//! - Implement state-specific behavior
//! - Manage state transitions with validation

#[derive(Debug, PartialEq, Clone)]
pub enum OrderState {
    Pending { items: Vec<String>, total: f64 },
    Processing { order_id: u64, items: Vec<String> },
    Shipped { order_id: u64, tracking_number: String },
    Delivered { order_id: u64, delivery_date: String },
    Cancelled { reason: String },
}

impl OrderState {
    /// Creates a new pending order
    pub fn new(items: Vec<String>, total: f64) -> Self  {
        todo!("Create a new pending order")
    }

    /// Processes the order
    pub fn process(self, order_id: u64) -> Result<Self, String>  {
        todo!("Process the order")
    }

    /// Ships the order
    pub fn ship(self, tracking_number: String) -> Result<Self, String>  {
        todo!("Ships the order")
    }

    /// Marks the order as delivered
    pub fn deliver(self, delivery_date: String) -> Result<Self, String>  {
        todo!("Marks the order as delivered")
    }

    /// Cancels the order
    pub fn cancel(self, reason: String) -> Result<Self, String>  {
        todo!("Cancels the order")
    }

    /// Returns the order status as a string
    pub fn status(&self) -> &'static str  {
        todo!("Return the order status as a string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_flow() {
        let order = OrderState::new(vec!["item1".to_string()], 100.0);
        let order = order.process(1).unwrap();
        assert_eq!(order.status(), "processing");

        let order = order.ship("TRACK123".to_string()).unwrap();
        assert_eq!(order.status(), "shipped");

        let order = order.deliver("2024-01-15".to_string()).unwrap();
        assert_eq!(order.status(), "delivered");
    }

    #[test]
    fn test_invalid_transitions() {
        let order = OrderState::new(vec!["item1".to_string()], 100.0);
        assert!(order.ship("TRACK123".to_string()).is_err());
    }

    #[test]
    fn test_cancellation() {
        let order = OrderState::new(vec!["item1".to_string()], 100.0);
        let order = order.cancel("Out of stock".to_string()).unwrap();
        assert_eq!(order.status(), "cancelled");
    }

    #[test]
    fn test_cannot_cancel_delivered() {
        let order = OrderState::new(vec!["item1".to_string()], 100.0);
        let order = order.process(1).unwrap();
        let order = order.ship("TRACK123".to_string()).unwrap();
        let order = order.deliver("2024-01-15".to_string()).unwrap();
        assert!(order.cancel("Changed mind".to_string()).is_err());
    }
}
