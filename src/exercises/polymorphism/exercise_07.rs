//! Exercise 07: Return Types with Traits - Return impl Trait from functions
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Return types that implement traits
//! - Understand impl Trait in return position
//! - Know limitations of impl Trait returns

pub trait Drawable {
    fn draw(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        todo!("Implement draw")
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        todo!("Implement draw")
    }
}

/// Returns a Circle that implements Drawable
pub fn create_circle(radius: f64) -> impl Drawable {
    todo!("Implement create_circle")
}

/// Returns a Rectangle that implements Drawable
pub fn create_rectangle(width: f64, height: f64) -> impl Drawable {
    todo!("Implement create_rectangle")
}

/// Function that uses the returned trait object
pub fn draw_shape(shape: &impl Drawable) -> String {
    todo!("Implement draw_shape")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_circle() {
        let circle = create_circle(5.0);
        assert!(circle.draw().contains("Circle"));
        assert!(circle.draw().contains("5"));
    }

    #[test]
    fn test_create_rectangle() {
        let rect = create_rectangle(10.0, 20.0);
        assert!(rect.draw().contains("Rectangle"));
        assert!(rect.draw().contains("10"));
        assert!(rect.draw().contains("20"));
    }

    #[test]
    fn test_draw_circle() {
        let circle = create_circle(3.5);
        let output = draw_shape(&circle);
        assert!(output.contains("Circle"));
    }

    #[test]
    fn test_draw_rectangle() {
        let rect = create_rectangle(8.0, 12.0);
        let output = draw_shape(&rect);
        assert!(output.contains("Rectangle"));
    }

    #[test]
    fn test_different_shapes() {
        let circle = create_circle(1.0);
        let rect = create_rectangle(2.0, 3.0);
        
        assert_ne!(circle.draw(), rect.draw());
    }

    #[test]
    fn test_shape_creation_and_drawing() {
        let shapes = vec![
            create_circle(5.0).draw(),
            create_rectangle(4.0, 6.0).draw(),
        ];
        
        assert_eq!(shapes.len(), 2);
        assert!(shapes[0].contains("Circle"));
        assert!(shapes[1].contains("Rectangle"));
    }
}
