pub mod problem {
    pub(crate) fn main() {
        // Error 1: Multiple mutable borrows
        let mut scores = vec![10, 20, 30];
        let first = &mut scores[0];
        *first += 5;
        println!("first: {}", first);
        let second = &mut scores[1];
        *second += 10;
        println!("second: {}", second);

        // Error 2: Mutable borrow while immutable borrow exists
        let mut text = String::from("hello");
        let len = text.len();
        text.push_str(" world");
        println!("Length was: {}", len);

        // Error 3: Using value after move in loop
        let messages = vec![
            String::from("Hello"),
            String::from("World"),
        ];

        for msg in &messages {
            println!("{}", msg);
        }

        println!("Number of messages: {}", messages.len());

        // Error 4: Returning reference to local variable
        let result = get_string();
        println!("{}", result);
    }

    fn get_string() -> String {
        let s = String::from("temporary");
        s
    }
}