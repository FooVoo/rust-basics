mod problem {
    // TODO: Create a trait called `TextOperation` with:
    // - apply(&self, text: &mut String)
    // - description(&self) -> &str

    struct Uppercase;
    struct Lowercase;
    struct Reverse;
    struct AppendText {
        suffix: String,
    }

    // TODO: Implement TextOperation for each struct

    struct TextEditor {
        content: String,
    }

    impl TextEditor {
        fn new(content: String) -> Self {
            TextEditor { content }
        }

        // TODO: Implement a method that applies an operation
        // fn apply_operation(&mut self, operation: &dyn TextOperation) {
        //     ...
        // }

        // TODO: Implement a method that returns the content
        // fn get_content(&self) -> &str {
        //     ...
        // }

        // TODO: Implement a method that applies multiple operations
        // fn apply_multiple(&mut self, operations: &[&dyn TextOperation]) {
        //     ...
        // }
    }

    pub(crate) fn main() {
        let mut editor = TextEditor::new(String::from("hello world"));

        // TODO: Create operations
        // TODO: Apply them to the editor
        // TODO: Print the final result
    }
}