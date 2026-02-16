pub mod problem {
    pub(crate) fn main() {
        // Challenge 1: Make this work without cloning
        let result;
        {
            let s = String::from("hello");
            result = get_first_word(&s);
            println!("{}", result);
        }

        // Challenge 2: Fix this function
        let x = String::from("short");
        let y = String::from("longer string");
        let result = longest_string(&x, &y);
        println!("{}", result);

        // Challenge 3: Make this work
        let mut data = vec![1, 2, 3, 4, 5];
        let sum = calculate_sum(&data);
        data.push(6);
        println!("Sum was: {}, new length: {}", sum, data.len());
        println!("New sum: {}", calculate_sum(&data));
    }

    fn get_first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    fn longest_string<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn calculate_sum(numbers: &Vec<i32>) -> i32 {
        numbers.iter().sum()
    }
}