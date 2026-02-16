mod problem {
    struct DataProcessor {
        data: Vec<i32>,
        processed: bool,
    }

    impl DataProcessor {
        fn new(data: Vec<i32>) -> Self {
            DataProcessor {
                data,
                processed: false,
            }
        }

        // TODO: Implement this method that returns a reference to the data
        // fn get_data(&self) -> ??? {
        //     &self.data
        // }

        // TODO: Implement this method that modifies the data
        // fn double_all(&mut self) {
        //     ...
        // }

        // TODO: Implement this method that returns a mutable reference
        // fn get_data_mut(&mut self) -> ??? {
        //     &mut self.data
        // }

        // TODO: Fix this method - it has borrowing issues
        fn process_and_print(&mut self) {
            let data_ref = &self.data;
            self.double_all(); // Problem here!
            println!("Data: {:?}", data_ref);
            self.processed = true;
        }
    }

    pub(crate) fn main() {
        let mut processor = DataProcessor::new(vec![1, 2, 3, 4, 5]);

        // TODO: Get immutable reference and print
        // TODO: Modify the data
        // TODO: Try to hold a reference while modifying (should fail)
        // TODO: Fix the process_and_print method
    }
}