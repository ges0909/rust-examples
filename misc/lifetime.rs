pub struct TextEditor {
    text: String, // Private member variable
}

impl TextEditor {
    pub fn new() -> TextEditor {
        TextEditor { text: String::new() }
    }

    // Modify text
    pub fn add_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    pub fn get_text<'a>(&'a self) -> &'a mut String {
        return &mut self.text;
    }

    pub fn reset(&mut self) {
        self.text = String::new();
    }
}

fn main() {
    let mut editor = TextEditor::new();

    editor.add_char('a');
    editor.add_char('b');
    editor.add_char('c');

    let my_txt = editor.get_text();

    editor.reset();

    println!("{}", my_txt); //Use after free. Not possible.
}
