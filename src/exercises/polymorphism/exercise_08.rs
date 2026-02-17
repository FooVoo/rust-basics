//! Exercise 08: Derived Traits - Use derive macro for common traits
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use #[derive] for common traits
//! - Understand which traits can be derived
//! - Implement custom traits alongside derived ones

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub trait Movable {
    fn move_by(&mut self, dx: i32, dy: i32);
}

impl Movable for Point {
    fn move_by(&mut self, dx: i32, dy: i32)  {
        todo!("Implement move_by")
    }
}

impl Movable for Line {
    fn move_by(&mut self, dx: i32, dy: i32)  {
        todo!("Implement move_by")
    }
}

pub trait Distance {
    fn distance(&self) -> f64;
}

impl Distance for Point {
    fn distance(&self) -> f64  {
        todo!("Implement distance")
    }
}

impl Distance for Line {
    fn distance(&self) -> f64  {
        todo!("Implement distance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_debug() {
        let point = Point { x: 3, y: 4 };
        let debug_str = format!("{:?}", point);
        assert!(debug_str.contains("3"));
        assert!(debug_str.contains("4"));
    }

    #[test]
    fn test_point_clone() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = point1.clone();
        assert_eq!(point1, point2);
    }

    #[test]
    fn test_point_equality() {
        let point1 = Point { x: 5, y: 5 };
        let point2 = Point { x: 5, y: 5 };
        let point3 = Point { x: 3, y: 4 };
        
        assert_eq!(point1, point2);
        assert_ne!(point1, point3);
    }

    #[test]
    fn test_point_movable() {
        let mut point = Point { x: 0, y: 0 };
        point.move_by(3, 4);
        assert_eq!(point, Point { x: 3, y: 4 });
    }

    #[test]
    fn test_line_movable() {
        let mut line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 3, y: 4 },
        };
        line.move_by(1, 1);
        assert_eq!(line.start, Point { x: 1, y: 1 });
        assert_eq!(line.end, Point { x: 4, y: 5 });
    }

    #[test]
    fn test_point_distance() {
        let point = Point { x: 3, y: 4 };
        assert_eq!(point.distance(), 5.0);
    }

    #[test]
    fn test_line_distance() {
        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 3, y: 4 },
        };
        assert_eq!(line.distance(), 5.0);
    }

    #[test]
    fn test_line_clone_and_modify() {
        let line1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 1, y: 1 },
        };
        let mut line2 = line1.clone();
        line2.move_by(5, 5);
        
        assert_ne!(line1, line2);
        assert_eq!(line1.start, Point { x: 0, y: 0 });
        assert_eq!(line2.start, Point { x: 5, y: 5 });
    }
}
