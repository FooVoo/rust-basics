//! Exercise 02: Match Expressions - Basic Pattern Matching
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use match expressions with enums
//! - Understand exhaustive pattern matching
//! - Return values from match arms

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// Returns the action to take for a given traffic light
pub fn get_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Slow down",
        TrafficLight::Green => "Go",
    }
}

/// Returns the next traffic light state
pub fn next_light(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Green => TrafficLight::Yellow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_action() {
        assert_eq!(get_action(TrafficLight::Red), "Stop");
        assert_eq!(get_action(TrafficLight::Yellow), "Slow down");
        assert_eq!(get_action(TrafficLight::Green), "Go");
    }

    #[test]
    fn test_next_light() {
        assert_eq!(next_light(TrafficLight::Red), TrafficLight::Green);
        assert_eq!(next_light(TrafficLight::Yellow), TrafficLight::Red);
        assert_eq!(next_light(TrafficLight::Green), TrafficLight::Yellow);
    }
}
