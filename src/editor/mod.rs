mod core;

use editor::core::Core;

pub struct Editor {
    core: Core,
}

pub fn new(buffer: String, line: usize, column: usize) -> Result<Editor, String> {
    match core::new(buffer, line, column) {
        Ok(editor) => Ok(Editor { core: editor }),
        Err(err) => Err(err),
    }
}

impl Editor {
    pub fn move_right(&mut self, n: usize) {
        self.core.move_right(n);
    }

    pub fn move_left(&mut self, n: usize) {
        self.core.move_left(n);
    }

    pub fn move_up(&mut self, n: usize) {
        self.core.move_up(n);
    }

    pub fn move_down(&mut self, n: usize) {
        self.core.move_down(n);
    }

    pub fn insert_at(&mut self, ch: char, line: usize, column: usize) {
        self.core.insert_at(ch, line, column);
    }

    pub fn insert_string_at(&mut self, s: &str, line: usize, column: usize) {
        self.core.insert_string_at(s, line, column);
    }

    pub fn delete_at(&mut self, line: usize, column: usize) {
        self.core.delete_at(line, column);
    }

    pub fn buffer(&self) -> String {
        self.core.buffer()
    }

    pub fn line(&self) -> usize {
        self.core.line()
    }

    pub fn column(&self) -> usize {
        self.core.column()
    }

    pub fn line_width(&self, n: usize) -> Option<usize> {
        self.core.line_width(n)
    }

    pub fn move_to_beginning(&mut self) {
        self.core.set_column(0);
    }

    pub fn move_to_end(&mut self) {
        let width = self.line_width(self.line()).unwrap();
        self.core.set_column(width);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_beginning() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = new(String::from(buffer), 1, 8).unwrap();
        editor.move_to_beginning();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        let mut editor = new(String::from(buffer), 0, 7).unwrap();
        editor.move_to_beginning();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);
    }

    #[test]
    fn test_move_to_end() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = new(String::from(buffer), 1, 8).unwrap();
        editor.move_to_end();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 13);

        let mut editor = new(String::from(buffer), 0, 7).unwrap();
        editor.move_to_end();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 13);
    }
}
