//! Exercise 04: Box in Collections - Store trait objects in vectors
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use Box for trait objects
//! - Store heterogeneous types in collections
//! - Understand dynamic dispatch with Box

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

/// Create a vector of boxed shapes.
pub fn create_shapes() -> Vec<Box<dyn Shape>> {
    vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
        Box::new(Circle { radius: 3.0 }),
    ]
}

/// Calculate total area of all shapes.
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 2.0 };
        let expected = std::f64::consts::PI * 4.0;
        assert!((circle.area() - expected).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(rect.area(), 12.0);
    }

    #[test]
    fn test_create_shapes() {
        let shapes = create_shapes();
        assert_eq!(shapes.len(), 3);
        assert_eq!(shapes[0].name(), "Circle");
        assert_eq!(shapes[1].name(), "Rectangle");
        assert_eq!(shapes[2].name(), "Circle");
    }

    #[test]
    fn test_total_area() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle { width: 2.0, height: 3.0 }),
        ];
        let total = total_area(&shapes);
        let expected = std::f64::consts::PI + 6.0;
        assert!((total - expected).abs() < 0.001);
    }
}
