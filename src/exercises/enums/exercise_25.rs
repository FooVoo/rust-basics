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
    pub fn new(items: Vec<String>, total: f64) -> Self {
        OrderState::Pending { items, total }
    }

    /// Processes the order
    pub fn process(self, order_id: u64) -> Result<Self, String> {
        match self {
            OrderState::Pending { items, .. } => {
                Ok(OrderState::Processing { order_id, items })
            }
            _ => Err("Can only process pending orders".to_string()),
        }
    }

    /// Ships the order
    pub fn ship(self, tracking_number: String) -> Result<Self, String> {
        match self {
            OrderState::Processing { order_id, .. } => Ok(OrderState::Shipped {
                order_id,
                tracking_number,
            }),
            _ => Err("Can only ship processing orders".to_string()),
        }
    }

    /// Marks the order as delivered
    pub fn deliver(self, delivery_date: String) -> Result<Self, String> {
        match self {
            OrderState::Shipped { order_id, .. } => Ok(OrderState::Delivered {
                order_id,
                delivery_date,
            }),
            _ => Err("Can only deliver shipped orders".to_string()),
        }
    }

    /// Cancels the order
    pub fn cancel(self, reason: String) -> Result<Self, String> {
        match self {
            OrderState::Delivered { .. } => Err("Cannot cancel delivered orders".to_string()),
            _ => Ok(OrderState::Cancelled { reason }),
        }
    }

    /// Returns the order status as a string
    pub fn status(&self) -> &'static str {
        match self {
            OrderState::Pending { .. } => "pending",
            OrderState::Processing { .. } => "processing",
            OrderState::Shipped { .. } => "shipped",
            OrderState::Delivered { .. } => "delivered",
            OrderState::Cancelled { .. } => "cancelled",
        }
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
