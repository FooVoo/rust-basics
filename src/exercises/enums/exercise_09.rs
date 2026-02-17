//! Exercise 09: Struct Variants - Enum with Named Fields
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Define enum variants with struct-like named fields
//! - Pattern match on struct variants
//! - Mix different variant types in one enum

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// Calculate the area of the shape
    pub fn area(&self) -> f64 {
        todo!("Implement area")
    }

    /// Returns the shape type as a string
    pub fn shape_type(&self) -> &'static str {
        todo!("Implement shape_type")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Shape::Circle { radius: 5.0 };
        assert!((circle.area() - 78.539816).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Shape::Rectangle {
            width: 4.0,
            height: 5.0,
        };
        assert_eq!(rect.area(), 20.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Shape::Triangle {
            base: 6.0,
            height: 4.0,
        };
        assert_eq!(triangle.area(), 12.0);
    }

    #[test]
    fn test_shape_type() {
        assert_eq!(Shape::Circle { radius: 1.0 }.shape_type(), "circle");
        assert_eq!(
            Shape::Rectangle {
                width: 1.0,
                height: 1.0
            }
            .shape_type(),
            "rectangle"
        );
        assert_eq!(
            Shape::Triangle {
                base: 1.0,
                height: 1.0
            }
            .shape_type(),
            "triangle"
        );
    }
}
