// TODO: Define a trait called `Shape` with two methods:
// - area(&self) -> f64
// - perimeter(&self) -> f64
pub mod problem {
    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    struct Triangle {
        side_a: f64,
        side_b: f64,
        side_c: f64,
    }

    // TODO: Implement the Shape trait for Circle, Rectangle, and Triangle
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }
    }

    impl Shape for Triangle {
        fn area(&self) -> f64 {
            self.side_a + self.side_b + self.side_c
        }

        fn perimeter(&self) -> f64 {
            self.side_a * self.side_b / 2.0
        }
    }

    // TODO: Write a function that takes any Shape and prints its area and perimeter
    fn print_shape_info(shape: &dyn Shape) {
        println!("{}", shape.area().to_string());
        println!("{}", shape.perimeter().to_string());
    }

    pub(crate) fn main() {
        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle { width: 4.0, height: 6.0 };
        let triangle = Triangle { side_a: 3.0, side_b: 4.0, side_c: 5.0 };

        // TODO: Call print_shape_info for each shape
        print_shape_info(&circle);
        print_shape_info(&rectangle);
        print_shape_info(&triangle);
    }
}