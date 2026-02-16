//! Exercise 19: Generic Event System - Create a generic event publisher/subscriber
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Build generic event systems
//! - Work with closures and FnMut
//! - Store generic callbacks

/// A generic event publisher that notifies subscribers.
pub struct Publisher<T> {
    subscribers: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> Publisher<T> {
    /// Creates a new publisher.
    pub fn new() -> Self {
        Publisher {
            subscribers: Vec::new(),
        }
    }

    /// Subscribes a callback to events.
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }

    /// Publishes an event to all subscribers.
    pub fn publish(&mut self, event: &T) {
        for callback in &mut self.subscribers {
            callback(event);
        }
    }

    /// Returns the number of subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }
}

impl<T> Default for Publisher<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A generic event bus with multiple event types.
pub struct EventBus<T> {
    handlers: Vec<Box<dyn FnMut(T)>>,
}

impl<T> EventBus<T> {
    /// Creates a new event bus.
    pub fn new() -> Self {
        EventBus {
            handlers: Vec::new(),
        }
    }

    /// Registers an event handler.
    pub fn on<F>(&mut self, handler: F)
    where
        F: FnMut(T) + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Emits an event to all handlers.
    pub fn emit(&mut self, event: T)
    where
        T: Clone,
    {
        for handler in &mut self.handlers {
            handler(event.clone());
        }
    }

    /// Returns the number of handlers.
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl<T> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_publisher_subscribe() {
        let mut publisher: Publisher<i32> = Publisher::new();
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();
        
        publisher.subscribe(move |_| {
            *counter_clone.borrow_mut() += 1;
        });
        
        assert_eq!(publisher.subscriber_count(), 1);
    }

    #[test]
    fn test_publisher_publish() {
        let mut publisher: Publisher<i32> = Publisher::new();
        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        
        publisher.subscribe(move |value| {
            *sum_clone.borrow_mut() += value;
        });
        
        publisher.publish(&10);
        publisher.publish(&20);
        
        assert_eq!(*sum.borrow(), 30);
    }

    #[test]
    fn test_publisher_multiple_subscribers() {
        let mut publisher: Publisher<String> = Publisher::new();
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));
        let count1_clone = count1.clone();
        let count2_clone = count2.clone();
        
        publisher.subscribe(move |_| {
            *count1_clone.borrow_mut() += 1;
        });
        
        publisher.subscribe(move |_| {
            *count2_clone.borrow_mut() += 1;
        });
        
        publisher.publish(&"event".to_string());
        
        assert_eq!(*count1.borrow(), 1);
        assert_eq!(*count2.borrow(), 1);
    }

    #[test]
    fn test_event_bus_on() {
        let mut bus: EventBus<i32> = EventBus::new();
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();
        
        bus.on(move |_| {
            *counter_clone.borrow_mut() += 1;
        });
        
        assert_eq!(bus.handler_count(), 1);
    }

    #[test]
    fn test_event_bus_emit() {
        let mut bus: EventBus<i32> = EventBus::new();
        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        
        bus.on(move |value| {
            *sum_clone.borrow_mut() += value;
        });
        
        bus.emit(10);
        bus.emit(20);
        
        assert_eq!(*sum.borrow(), 30);
    }

    #[test]
    fn test_event_bus_multiple_handlers() {
        let mut bus: EventBus<String> = EventBus::new();
        let messages = Rc::new(RefCell::new(Vec::new()));
        let msg1 = messages.clone();
        let msg2 = messages.clone();
        
        bus.on(move |s| {
            msg1.borrow_mut().push(format!("Handler1: {}", s));
        });
        
        bus.on(move |s| {
            msg2.borrow_mut().push(format!("Handler2: {}", s));
        });
        
        bus.emit("test".to_string());
        
        assert_eq!(messages.borrow().len(), 2);
    }

    #[test]
    fn test_publisher_string_events() {
        let mut publisher: Publisher<String> = Publisher::new();
        let messages = Rc::new(RefCell::new(Vec::new()));
        let messages_clone = messages.clone();
        
        publisher.subscribe(move |msg| {
            messages_clone.borrow_mut().push(msg.clone());
        });
        
        publisher.publish(&"hello".to_string());
        publisher.publish(&"world".to_string());
        
        assert_eq!(messages.borrow().len(), 2);
    }
}
