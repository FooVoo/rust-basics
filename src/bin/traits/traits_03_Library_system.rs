pub mod problem {
    // TODO: Create a trait called `Borrowable` with these methods:
    // - borrow_item(&mut self) -> Result<String, String>
    // - return_item(&mut self) -> Result<String, String>
    // - is_available(&self) -> bool
    // - get_title(&self) -> &str

    struct Item {
        title: String,
        is_checked_out: bool,
    }

    trait Borrowable {
        fn borrow_item(&mut self) -> Result<String, String>;
        fn return_item(&mut self) -> Result<String, String>;
        fn is_available(&self) -> bool;
        fn get_title(&self) -> &str;
    }

    struct Book {
        title: String,
        author: String,
        is_checked_out: bool,
    }

    struct DVD {
        title: String,
        duration_minutes: u32,
        is_checked_out: bool,
    }

    struct Magazine {
        title: String,
        issue_number: u32,
        is_checked_out: bool,
    }

    impl Borrowable for Book {
        fn borrow_item(&mut self) -> Result<String, String> {
            if self.is_checked_out {
               return Err("Item already checked out".to_string());
            }
            self.is_checked_out = true;
            Ok(format!("You have borrowed the book: {} from {}", self.title, self.author))
        }
        fn return_item(&mut self) -> Result<String, String> {
            if !self.is_checked_out {
               return Err("Item not checked out".to_string());
            }
            self.is_checked_out = false;
            Ok(format!("You have returned the book: {}", self.title))
        }
        fn is_available(&self) -> bool {
            !self.is_checked_out
        }
        fn get_title(&self) -> &str {
            &self.title
        }
    }

    impl Borrowable for DVD {
        fn borrow_item(&mut self) -> Result<String, String> {
            if self.is_checked_out {
               return Err("Item already checked out".to_string());
            }
            self.is_checked_out = true;
            Ok(format!("You have borrowed the DVD: {}, with duration of {}", self.title, self.duration_minutes))
        }
        fn return_item(&mut self) -> Result<String, String> {
            if !self.is_checked_out {
               return Err("Item not checked out".to_string());
            }
            self.is_checked_out = false;
            Ok(format!("You have returned the DVD: {}", self.title))
        }
        fn is_available(&self) -> bool {
            !self.is_checked_out
        }
        fn get_title(&self) -> &str {
            &self.title
        }
    }

    impl Borrowable for Magazine {
        fn borrow_item(&mut self) -> Result<String, String> {
            if self.is_checked_out {
               return Err("Item already checked out".to_string());
            }
            self.is_checked_out = true;
            Ok(format!("You have borrowed the magazine: {} with number {}", self.title, self.issue_number))
        }
        fn return_item(&mut self) -> Result<String, String> {
            if !self.is_checked_out {
                return Err("Item not checked out".to_string());
            }
            self.is_checked_out = false;
            Ok(format!("You have returned the magazine: {}", self.title))
        }
        fn is_available(&self) -> bool {
            !self.is_checked_out
        }
        fn get_title(&self) -> &str {
            &self.title
        }
    }

    // TODO: Create a function that attempts to borrow multiple items
    // If an item is already checked out, it should return an error
    // fn borrow_multiple(items: ???) -> Result<(), String> {
    //     ...
    // }

    fn borrow_multiple(items: &mut [&mut dyn Borrowable]) -> Result<(), String> {
        for item in items.iter() {
            if !item.is_available() {
               return Err(format!("Item {} is not available", item.get_title()));
            }
        }

        for item in items.iter_mut() {
            println!("{}", item.borrow_item()?);
        }

        Ok(())
    }

    pub(crate) fn main() {
        let mut book = Book {
            title: String::from("The Rust Book"),
            author: String::from("Steve Klabnik"),
            is_checked_out: false,
        };

        let mut dvd = DVD {
            title: String::from("Rust: The Movie"),
            duration_minutes: 120,
            is_checked_out: false,
        };

        let mut items: Vec<&mut dyn Borrowable> = vec![&mut dvd, &mut book];
        borrow_multiple(&mut items).unwrap();
    }
}