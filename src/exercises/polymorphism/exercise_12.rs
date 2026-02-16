//! Exercise 12: Trait Inheritance - Create trait hierarchies
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Define trait inheritance with supertraits
//! - Implement trait hierarchies
//! - Understand trait bounds with supertrait requirements

pub trait Shape {
    fn area(&self) -> f64;
}

pub trait ColoredShape: Shape {
    fn color(&self) -> &str;
    
    fn describe(&self) -> String {
        format!("A {} shape with area {}", self.color(), self.area())
    }
}

pub trait NamedColoredShape: ColoredShape {
    fn name(&self) -> &str;
    
    fn full_description(&self) -> String {
        format!("{}: {}", self.name(), self.describe())
    }
}

pub struct ColoredCircle {
    pub radius: f64,
    pub color: String,
    pub name: String,
}

impl Shape for ColoredCircle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl ColoredShape for ColoredCircle {
    fn color(&self) -> &str {
        &self.color
    }
}

impl NamedColoredShape for ColoredCircle {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct ColoredRectangle {
    pub width: f64,
    pub height: f64,
    pub color: String,
}

impl Shape for ColoredRectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl ColoredShape for ColoredRectangle {
    fn color(&self) -> &str {
        &self.color
    }
}

/// Function requiring ColoredShape (which requires Shape)
pub fn print_colored_shape<T: ColoredShape>(shape: &T) -> String {
    format!("Color: {}, Area: {}", shape.color(), shape.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colored_circle_shape() {
        let circle = ColoredCircle {
            radius: 1.0,
            color: "red".to_string(),
            name: "RedCircle".to_string(),
        };
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_colored_circle_color() {
        let circle = ColoredCircle {
            radius: 2.0,
            color: "blue".to_string(),
            name: "BlueCircle".to_string(),
        };
        assert_eq!(circle.color(), "blue");
    }

    #[test]
    fn test_colored_circle_describe() {
        let circle = ColoredCircle {
            radius: 1.0,
            color: "green".to_string(),
            name: "GreenCircle".to_string(),
        };
        let desc = circle.describe();
        assert!(desc.contains("green"));
        assert!(desc.contains("shape"));
    }

    #[test]
    fn test_named_colored_shape() {
        let circle = ColoredCircle {
            radius: 1.0,
            color: "yellow".to_string(),
            name: "Sun".to_string(),
        };
        assert_eq!(circle.name(), "Sun");
        let full_desc = circle.full_description();
        assert!(full_desc.contains("Sun"));
        assert!(full_desc.contains("yellow"));
    }

    #[test]
    fn test_colored_rectangle() {
        let rect = ColoredRectangle {
            width: 5.0,
            height: 10.0,
            color: "purple".to_string(),
        };
        assert_eq!(rect.area(), 50.0);
        assert_eq!(rect.color(), "purple");
    }

    #[test]
    fn test_print_colored_shape() {
        let circle = ColoredCircle {
            radius: 1.0,
            color: "orange".to_string(),
            name: "OrangeCircle".to_string(),
        };
        let output = print_colored_shape(&circle);
        assert!(output.contains("orange"));
        assert!(output.contains("Area:"));
    }

    #[test]
    fn test_trait_hierarchy() {
        let circle = ColoredCircle {
            radius: 2.0,
            color: "black".to_string(),
            name: "BlackCircle".to_string(),
        };
        
        // Can use all levels of the hierarchy
        let _area: f64 = circle.area();
        let _color: &str = circle.color();
        let _name: &str = circle.name();
        let _desc = circle.describe();
        let full = circle.full_description();
        
        assert!(full.contains("BlackCircle"));
        assert!(full.contains("black"));
    }
}
