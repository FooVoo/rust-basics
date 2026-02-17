//! Exercise 15: Trait Methods Calling Other Trait Methods
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Define traits with methods that call other trait methods
//! - Create composable trait APIs
//! - Understand self in trait methods

pub trait Measurable {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width() + self.height())
    }
    
    fn is_square(&self) -> bool {
        (self.width() - self.height()).abs() < 0.001
    }
    
    fn is_larger_than(&self, other: &impl Measurable) -> bool {
        self.area() > other.area()
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Measurable for Rectangle {
    fn width(&self) -> f64 {
        self.width
    }
    
    fn height(&self) -> f64 {
        self.height
    }
}

pub struct Monitor {
    pub width_inches: f64,
    pub height_inches: f64,
}

impl Measurable for Monitor {
    fn width(&self) -> f64 {
        self.width_inches
    }
    
    fn height(&self) -> f64 {
        self.height_inches
    }
    
    // Override area to compute diagonal
    fn area(&self) -> f64 {
        (self.width_inches.powi(2) + self.height_inches.powi(2)).sqrt()
    }
}

pub fn compare_sizes<T: Measurable, U: Measurable>(a: &T, b: &U) -> String {
    if a.area() > b.area() {
        format!("First is larger: {} vs {}", a.area(), b.area())
    } else if a.area() < b.area() {
        format!("Second is larger: {} vs {}", b.area(), a.area())
    } else {
        "Both are equal in size".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_dimensions() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(rect.width(), 10.0);
        assert_eq!(rect.height(), 5.0);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 4.0, height: 6.0 };
        assert_eq!(rect.area(), 24.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle { width: 3.0, height: 7.0 };
        assert_eq!(rect.perimeter(), 20.0);
    }

    #[test]
    fn test_rectangle_is_square() {
        let square = Rectangle { width: 5.0, height: 5.0 };
        let rect = Rectangle { width: 5.0, height: 3.0 };
        
        assert!(square.is_square());
        assert!(!rect.is_square());
    }

    #[test]
    fn test_is_larger_than() {
        let large = Rectangle { width: 10.0, height: 10.0 };
        let small = Rectangle { width: 2.0, height: 2.0 };
        
        assert!(large.is_larger_than(&small));
        assert!(!small.is_larger_than(&large));
    }

    #[test]
    fn test_monitor_dimensions() {
        let monitor = Monitor {
            width_inches: 24.0,
            height_inches: 13.5,
        };
        assert_eq!(monitor.width(), 24.0);
        assert_eq!(monitor.height(), 13.5);
    }

    #[test]
    fn test_monitor_area_override() {
        let monitor = Monitor {
            width_inches: 3.0,
            height_inches: 4.0,
        };
        // Diagonal is 5.0 (3-4-5 triangle)
        assert_eq!(monitor.area(), 5.0);
    }

    #[test]
    fn test_compare_sizes() {
        let rect1 = Rectangle { width: 10.0, height: 10.0 };
        let rect2 = Rectangle { width: 5.0, height: 5.0 };
        
        let comparison = compare_sizes(&rect1, &rect2);
        assert!(comparison.contains("First is larger"));
    }

    #[test]
    fn test_compare_different_types() {
        let rect = Rectangle { width: 5.0, height: 5.0 };
        let monitor = Monitor { width_inches: 3.0, height_inches: 4.0 };
        
        // rect area = 25, monitor area (diagonal) = 5
        let comparison = compare_sizes(&rect, &monitor);
        assert!(comparison.contains("First is larger"));
    }
}
