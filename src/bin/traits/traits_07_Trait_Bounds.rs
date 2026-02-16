mod problem {
    // TODO: Create a Drawable trait with a draw(&self) method

    struct Circle {
        radius: f64,
    }

    struct Square {
        side: f64,
    }

    // TODO: Implement Drawable for Circle and Square

    // TODO: Write a generic function that takes any Drawable and draws it twice
    // fn draw_twice<???>(item: ???) {
    //     ...
    // }

    // TODO: Create a Comparable trait with a compare(&self, other: &Self) -> i32 method
    // Return: -1 if self < other, 0 if equal, 1 if self > other

    struct Person {
        name: String,
        age: u32,
    }

    // TODO: Implement Comparable for Person (compare by age)

    // TODO: Write a generic function that takes two Comparable items and returns the larger one
    // fn get_larger<???>(a: ???, b: ???) -> ??? {
    //     ...
    // }

    pub(crate) fn main() {
        // TODO: Test your implementations
    }
}