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
pub use editor::core::Position;

use std::ops::Range;

/// A fundamental two-dimensional editor which has text as its buffer.
pub struct Editor {
    core: Core,
}

impl Editor {
    /// Creates a new `Editor` which has a buffer and a position of the cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("edit here", 0, 4).unwrap();
    /// ```
    pub fn new(buffer: &str, line: usize, column: usize) -> Result<Editor, String> {
        Core::new(buffer, line, column).and_then(|core| Ok(Editor { core }))
    }

    /// Moves a cursor by `n` characters rightward.
    ///
    /// If the cursor will be out of the range, it is moved to the rightmost column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("edit here", 0, 4).unwrap();
    /// editor.move_right(1);
    /// assert_eq!(editor.column(), 5);
    ///
    /// editor.move_right(1000);
    /// assert_eq!(editor.column(), 9);
    /// ```
    pub fn move_right(&mut self, n: usize) {
        self.core.move_right(n);
    }

    /// Moves a cursor by `n` characters leftward.
    ///
    /// If the cursor will be out of the range, it is moved to the leftmost column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("edit here", 0, 4).unwrap();
    /// editor.move_left(1);
    /// assert_eq!(editor.column(), 3);
    ///
    /// editor.move_left(1000);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_left(&mut self, n: usize) {
        self.core.move_left(n);
    }

    /// Moves a cursor by `n` lines upward.
    ///
    /// If the cursor will be out of the range, it is moved to the uppermost line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new(" 1 \n 2 \n 3 \n 4 ", 3, 0).unwrap();
    /// editor.move_up(1);
    /// assert_eq!(editor.line(), 2);
    ///
    /// editor.move_up(1000);
    /// assert_eq!(editor.line(), 0);
    /// ```
    pub fn move_up(&mut self, n: usize) {
        self.core.move_up(n);
    }

    /// Moves a cursor by `n` lines downward.
    ///
    /// If the cursor will be out of the range, it is moved to the downmost line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new(" 1 \n 2 \n 3 \n 4 ", 0, 0).unwrap();
    /// editor.move_down(1);
    /// assert_eq!(editor.line(), 1);
    ///
    /// editor.move_down(1000);
    /// assert_eq!(editor.line(), 3);
    /// ```
    pub fn move_down(&mut self, n: usize) {
        self.core.move_down(n);
    }

    /// Inserts a character into the buffer at a character position.
    ///
    /// If a position is out of the range, nothing happens.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("insert on character", 0, 0).unwrap();
    /// editor.insert_at('e', 0, 9);
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "insert one character");
    /// ```
    pub fn insert_at(&mut self, ch: char, line: usize, column: usize) {
        self.core.insert_at(ch, line, column);
    }

    /// Inserts a string into the buffer at a character position.
    ///
    /// If a position is out of the range, nothing happens.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("insert", 0, 0).unwrap();
    /// editor.insert_string_at(" string", 0, 6);
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "insert string");
    /// ```
    pub fn insert_string_at(&mut self, s: &str, line: usize, column: usize) {
        self.core.insert_string_at(s, line, column);
    }

    /// Deletes a `char` from the buffer at a character position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("abc", 0, 3).unwrap();
    /// editor.delete_at(0, 1);
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "ac");
    ///
    /// assert_eq!(editor.column(), 2);
    /// ```
    pub fn delete_at(&mut self, line: usize, column: usize) {
        self.core.delete_at(line, column);
    }

    /// Deletes characters from the buffer in a character range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// use edit::editor::Position;
    /// let mut editor = Editor::new("abcdefg \n hijk", 0, 3).unwrap();
    /// editor.delete_range(Position { line: 0, column: 4 }..Position { line: 1, column: 3 });
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "abcdjk");
    /// ```
    pub fn delete_range(&mut self, range: Range<Position>) {
        self.core.delete_range(range);
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

    /// Returns the count of characters of line `n`.
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

    /// Moves a cursor to the beginning of the first line.
    pub fn move_to_beginning_of_first_line(&mut self) {
        self.core.set_column(0);
        self.core.set_line(0);
    }

    /// Moves a cursor to the beginning of the last line.
    pub fn move_to_beginning_of_last_line(&mut self) {
        self.core.set_column(0);
        let lines = self.core.line_count();
        self.core.set_line(lines - 1);
    }

    /// Moves a cursor to the beginning of the middle line.
    /// If the number of the lines is even, the upper line is selected.
    pub fn move_to_beginning_of_middle_line(&mut self) {
        self.set_column(0);
        let mut lines = self.core.line_count();
        if lines % 2 == 0 {
            lines -= 1;
        }
        self.set_line(lines / 2);
    }

    /// Moves a cursor to the beginning of a next keyword.
    pub fn move_to_beginning_of_next_keyword(&mut self) {
        let pos = self.next_word_position();
        if pos.is_none() {
            return;
        }
        self.set_line(pos.unwrap().0);
        self.set_column(pos.unwrap().1);
    }

    fn next_word_position(&self) -> Option<(usize, usize)> {
        let off = self.core.current_offset();
        let buffer = self.core.buffer();
        let mut line = self.line();
        let mut i = self.column();
        for ch in buffer[off..].iter() {
            if *ch == '\n' {
                line += 1;
                i = 0;
            } else if ch.is_alphabetic() {
                return Some((line, i));
            } else {
                i += 1;
            }
        }
        return None;
    }

    fn current_line_buffer(&self) -> &[char] {
        let buffer = self.core.buffer();
        let beginning = self.core.offset(self.line(), 0).unwrap();
        let end = self.core
            .offset(self.line(), self.core.current_line_width())
            .unwrap();
        &buffer[beginning..end]
    }

    fn first_non_blank(&self) -> Option<usize> {
        let line = self.current_line_buffer();
        for (i, ch) in line.iter().enumerate() {
            if !ch.is_whitespace() {
                return Some(i);
            }
        }
        return None;
    }

    /// Moves a cursor to the first non-blank character.
    pub fn move_to_beginning_of_non_blank(&mut self) {
        let pos = self.first_non_blank();
        if pos.is_none() {
            return;
        }
        self.set_column(pos.unwrap());
    }

    fn last_non_blank(&self) -> Option<usize> {
        let line = self.current_line_buffer();
        for (i, ch) in line.iter().enumerate().rev() {
            if !ch.is_whitespace() {
                return Some(i + 1);
            }
        }
        return None;
    }

    /// Moves a cursor to the last non-blank character.
    pub fn move_to_end_of_non_blank(&mut self) {
        let pos = self.last_non_blank();
        if pos.is_none() {
            return;
        }
        self.set_column(pos.unwrap());
    }

    /// Moves a cursor to a line.
    ///
    /// If a position is out of the range, nothing happens.
    pub fn set_line(&mut self, n: usize) {
        return self.core.set_line(n);
    }

    /// Moves a cursor to a column.
    ///
    /// If a position is out of the range, nothing happens.
    pub fn set_column(&mut self, n: usize) {
        return self.core.set_column(n);
    }

    /// Moves a cursor to the beginning of the upper line.
    pub fn move_to_beginning_of_upper_line(&mut self) {
        self.set_column(0);
        self.move_down(1);
    }

    /// Moves a cursor to the beginning of the lower line.
    pub fn move_to_beginning_of_lower_line(&mut self) {
        self.set_column(0);
        self.move_up(1);
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
    fn test_move_to_beginning_of_first_line() {
        let buffer = "aaa\nbbb\nccc\ndd";
        let mut editor = Editor::new(buffer, 1, 3).unwrap();
        editor.move_to_beginning_of_first_line();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);

        let mut editor = Editor::new(buffer, 3, 0).unwrap();
        editor.move_to_beginning_of_first_line();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);
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

    #[test]
    fn test_move_to_beginning_of_middle_line() {
        let buffer = "aaa\nbbb\nccc\ndd";
        let mut editor = Editor::new(buffer, 1, 3).unwrap();
        editor.move_to_beginning_of_middle_line();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        let mut editor = Editor::new(buffer, 3, 2).unwrap();
        editor.move_to_beginning_of_middle_line();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        let buffer = "aaa\nbbb\nccc";
        let mut editor = Editor::new(buffer, 1, 3).unwrap();
        editor.move_to_beginning_of_middle_line();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        let buffer = "aaa";
        let mut editor = Editor::new(buffer, 0, 2).unwrap();
        editor.move_to_beginning_of_middle_line();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);
    }

    #[test]
    fn test_move_to_beginning_of_next_keyword() {
        let buffer = "  aaa  ";
        let mut editor = Editor::new(buffer, 0, 1).unwrap();
        editor.move_to_beginning_of_next_keyword();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 2);

        let buffer = "  aaa \n    bbb  ";
        let mut editor = Editor::new(buffer, 0, 5).unwrap();
        editor.move_to_beginning_of_next_keyword();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 4);
    }

    #[test]
    fn test_move_to_beginning_of_non_blank() {
        let buffer = "  aaa  ";
        let mut editor = Editor::new(buffer, 0, 6).unwrap();
        editor.move_to_beginning_of_non_blank();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 2);
    }

    #[test]
    fn test_move_to_end_of_non_blank() {
        let buffer = "  aaa  ";
        let mut editor = Editor::new(buffer, 0, 2).unwrap();
        editor.move_to_end_of_non_blank();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 5);
    }

    #[test]
    fn test_move_to_beginning_of_upper_line() {
        let buffer = "aaa\nbbb\nccc";
        let mut editor = Editor::new(buffer, 0, 3).unwrap();
        editor.move_to_beginning_of_upper_line();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        editor.move_to_beginning_of_upper_line();
        assert_eq!(editor.line(), 2);
        assert_eq!(editor.column(), 0);

        editor.move_to_beginning_of_upper_line();
        assert_eq!(editor.line(), 2);
        assert_eq!(editor.column(), 0);
    }

    #[test]
    fn test_move_to_beginning_of_lower_line() {
        let buffer = "aaa\nbbb\nccc";
        let mut editor = Editor::new(buffer, 2, 3).unwrap();
        editor.move_to_beginning_of_lower_line();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 0);

        editor.move_to_beginning_of_lower_line();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);

        editor.move_to_beginning_of_lower_line();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 0);
    }
}
