//! A fundamental two-dimensional editor which has text as its buffer.
//!
//! This module contains the `Editor` type.
//!
//! # Examples
//!
//! ```
//! use edit::editor::Editor;
//!
//! let mut editor = Editor::new("abcabc\ndefdef", 1, 3).unwrap();
//! editor.move_up(1);
//! editor.move_left(2);
//! editor.insert_string_at("\nghighi" , 1, 6);
//!
//! let buffer: String = editor.buffer().iter().collect();
//! assert_eq!(buffer, "abcabc\ndefdef\nghighi");
//! ```

mod core;

use editor::core::Core;

/// A fundamental two-dimensional editor which has text as its buffer.
pub struct Editor {
    core: Core,
}

impl Editor {
    /// Creates a new `Editor` which has a buffer and a position of the cursor.
    pub fn new(buffer: &str, line: usize, column: usize) -> Result<Editor, String> {
        Core::new(buffer, line, column).and_then(|core| Ok(Editor { core }))
    }

    /// Moves a cursor by `n` bytes rightward.
    ///
    /// If the cursor will be out of the range, it is moved to the rightmost column.
    pub fn move_right(&mut self, n: usize) {
        self.core.move_right(n);
    }

    /// Moves a cursor by `n` bytes leftward.
    ///
    /// If the cursor will be out of the range, it is moved to the leftmost column.
    pub fn move_left(&mut self, n: usize) {
        self.core.move_left(n);
    }

    /// Moves a cursor by `n` lines upward.
    ///
    /// If the cursor will be out of the range, it is moved to the uppermost line.
    pub fn move_up(&mut self, n: usize) {
        self.core.move_up(n);
    }

    /// Moves a cursor by `n` lines downward.
    ///
    /// If the cursor will be out of the range, it is moved to the downmost line.
    pub fn move_down(&mut self, n: usize) {
        self.core.move_down(n);
    }

    /// Inserts a character into the buffer at a byte position.
    ///
    /// If a position is out of the range, nothing happens.
    pub fn insert_at(&mut self, ch: char, line: usize, column: usize) {
        self.core.insert_at(ch, line, column);
    }

    /// Inserts a string into the buffer at a byte position.
    ///
    /// If a position is out of the range, nothing happens.
    pub fn insert_string_at(&mut self, s: &str, line: usize, column: usize) {
        self.core.insert_string_at(s, line, column);
    }

    /// Deletes a `char` from the buffer at a byte position.
    pub fn delete_at(&mut self, line: usize, column: usize) {
        self.core.delete_at(line, column);
    }

    /// Shows the content of the buffer.
    pub fn buffer(&self) -> &[char] {
        self.core.buffer()
    }

    /// Returns the line of the position.
    pub fn line(&self) -> usize {
        self.core.line()
    }

    /// Returns the column of the position.
    pub fn column(&self) -> usize {
        self.core.column()
    }

    /// Returns the count of bytes of line `n`.
    ///
    /// Returns `None` if `n` is out of the range.
    pub fn line_width(&self, n: usize) -> Option<usize> {
        self.core.line_width(n)
    }

    /// Moves a cursor to the beginning of the current line.
    pub fn move_to_beginning(&mut self) {
        self.core.set_column(0);
    }

    /// Moves a cursor to the end of the current line.
    pub fn move_to_end(&mut self) {
        let width = self.line_width(self.line()).unwrap();
        self.core.set_column(width);
    }

    /// Moves a cursor to the beginning of the last line.
    pub fn move_to_beginning_of_last_line(&mut self) {
        self.core.set_column(0);
        let lines = self.core.line_count();
        self.core.set_line(lines - 1);
    }
}

impl Clone for Editor {
    fn clone(&self) -> Editor {
        Editor { core: self.core.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_beginning() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = Editor::new(buffer, 1, 8).unwrap();
        editor.move_to_beginning();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        let mut editor = Editor::new(buffer, 0, 7).unwrap();
        editor.move_to_beginning();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);
    }

    #[test]
    fn test_move_to_end() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = Editor::new(buffer, 1, 8).unwrap();
        editor.move_to_end();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 13);

        let mut editor = Editor::new(buffer, 0, 7).unwrap();
        editor.move_to_end();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 13);
    }

    #[test]
    fn test_move_to_beginning_of_last_line() {
        let buffer = "aaa\nbbb\nccc\ndd";
        let mut editor = Editor::new(buffer, 1, 3).unwrap();
        editor.move_to_beginning_of_last_line();
        assert_eq!(editor.line(), 3);
        assert_eq!(editor.column(), 0);

        let mut editor = Editor::new(buffer, 3, 2).unwrap();
        editor.move_to_beginning_of_last_line();
        assert_eq!(editor.line(), 3);
        assert_eq!(editor.column(), 0);
    }
}
