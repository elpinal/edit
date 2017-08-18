//! A fundamental two-dimensional editor which has text as its buffer.
//!
//! This module contains the `Editor` type.
//!
//! # Examples
//!
//! ```
//! use edit::editor::Editor;
//!
//! let mut editor = Editor::new("abcabc\n\
//!                               defdef", 1, 3).unwrap();
//! editor.move_up(1);
//! editor.move_left(2);
//! editor.insert_string_at("\nghighi" , 1, 6);
//!
//! let buffer: String = editor.buffer().iter().collect();
//! assert_eq!(buffer, "abcabc\n\
//!                     defdef\n\
//!                     ghighi");
//! ```

mod core;

use editor::core::Core;
pub use editor::core::Position;

use std::ops::Range;

/// A pair of parentheses.
///
/// # Examples
///
/// ```
/// use edit::editor::Paren;
///
/// let p = Paren {
///     open: '(',
///     close: ')',
/// };
/// ```
pub struct Paren {
    /// A character which is a open parenthesis.
    pub open: char,
    /// A character which is a close parenthesis.
    pub close: char,
}

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
    ///
    /// let editor = Editor::new("edit here", 1, 0);
    /// // invalid position!
    /// assert!(editor.is_err());
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
    /// editor.delete_range(Position::new(0, 4)..Position::new(1, 3));
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "abcdjk");
    /// ```
    pub fn delete_range(&mut self, range: Range<Position>) {
        self.core.delete_range(range);
    }

    /// Deletes a line from the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a a\n\
    ///                               b b b", 1, 3).unwrap();
    /// editor.delete_line(0);
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "b b b");
    /// assert_eq!(editor.line(), 0);
    /// assert_eq!(editor.column(), 3);
    ///
    /// editor.delete_line(0);
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "");
    /// assert_eq!(editor.line(), 0);
    /// assert_eq!(editor.column(), 0);
    ///
    /// let mut editor = Editor::new("a b", 0, 2).unwrap();
    /// editor.delete_line(0);
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "");
    /// assert_eq!(editor.line(), 0);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn delete_line(&mut self, line: usize) {
        let end = if self.is_last_line(line) {
            Position::new(line, self.line_width(line).unwrap())
        } else {
            Position::new(line + 1, 0)
        };
        self.delete_range(Position::new(line, 0)..end);
    }

    fn is_last_line(&self, line: usize) -> bool {
        line == self.core.line_count() - 1
    }

    /// Shows the content of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("abc", 0, 0).unwrap();
    ///
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "abc");
    /// ```
    pub fn buffer(&self) -> &[char] {
        self.core.buffer()
    }

    /// Returns the line of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("A\nA\nA\nA\nA", 3, 0).unwrap();
    /// assert_eq!(editor.line(), 3);
    /// ```
    pub fn line(&self) -> usize {
        self.core.line()
    }

    /// Returns the column of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("A\nA\nAAAAA\nA\nA", 2, 4).unwrap();
    /// assert_eq!(editor.column(), 4);
    /// ```
    pub fn column(&self) -> usize {
        self.core.column()
    }

    /// Returns the count of characters of line `n`.
    ///
    /// Returns `None` if `n` is out of the range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("A\nA\nAAAAA\nA\nA", 2, 4).unwrap();
    /// assert_eq!(editor.line_width(0), Some(1));
    /// assert_eq!(editor.line_width(2), Some(5));
    /// assert_eq!(editor.line_width(4), Some(1));
    /// assert_eq!(editor.line_width(5), None);
    /// ```
    pub fn line_width(&self, n: usize) -> Option<usize> {
        self.core.line_width(n)
    }

    /// Moves a cursor to the beginning of the current line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\nbbb", 1, 2).unwrap();
    /// editor.move_to_beginning();
    /// assert_eq!(editor.line(), 1);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_to_beginning(&mut self) {
        self.set_column(0);
    }

    /// Moves a cursor to the end of the current line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\nbbb", 1, 2).unwrap();
    /// editor.move_to_end();
    /// assert_eq!(editor.line(), 1);
    /// assert_eq!(editor.column(), 3);
    /// ```
    pub fn move_to_end(&mut self) {
        let width = self.core.current_line_width();
        self.set_column(width);
    }

    /// Moves a cursor to the beginning of the first line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\nbbb", 1, 2).unwrap();
    /// editor.move_to_beginning_of_first_line();
    /// assert_eq!(editor.line(), 0);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_to_beginning_of_first_line(&mut self) {
        self.set_column(0);
        self.set_line(0);
    }

    /// Moves a cursor to the beginning of the last line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\nbbb", 0, 2).unwrap();
    /// editor.move_to_beginning_of_last_line();
    /// assert_eq!(editor.line(), 1);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_to_beginning_of_last_line(&mut self) {
        self.set_column(0);
        let lines = self.core.line_count();
        self.set_line(lines - 1);
    }

    /// Moves a cursor to the beginning of the middle line.
    /// If the number of the lines is even, the upper line is selected.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\nbbb", 0, 2).unwrap();
    /// editor.move_to_beginning_of_middle_line();
    /// assert_eq!(editor.line(), 0);
    /// assert_eq!(editor.column(), 0);
    ///
    /// let mut editor = Editor::new("aaa\n\
    ///                               bbb\n\
    ///                               ccc\n\
    ///                               ddd\n\
    ///                               eee", 0, 2).unwrap();
    /// editor.move_to_beginning_of_middle_line();
    /// assert_eq!(editor.line(), 2);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_to_beginning_of_middle_line(&mut self) {
        self.set_column(0);
        let mut lines = self.core.line_count();
        if lines % 2 == 0 {
            lines -= 1;
        }
        self.set_line(lines / 2);
    }

    /// Moves a cursor to the beginning of a next keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("   bbb", 0, 2).unwrap();
    /// editor.move_to_beginning_of_next_keyword();
    /// assert_eq!(editor.column(), 3);
    /// ```
    pub fn move_to_beginning_of_next_keyword(&mut self) {
        if let Some(pos) = self.next_keyword_position() {
            self.set_line(pos.line);
            self.set_column(pos.column);
        }
    }

    /// Returns a position at the beginning of a next keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// use edit::editor::Position;
    /// let editor = Editor::new("aa \n\
    ///                           bb ", 0, 0).unwrap();
    /// let pos = editor.next_keyword_position();
    /// assert_eq!(pos, Some(Position::new(1, 0)));
    /// ```
    pub fn next_keyword_position(&self) -> Option<Position> {
        self.core.next_keyword_position()
    }

    /// Moves a cursor to the beginning of a previous keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\n bbb", 1, 3).unwrap();
    /// editor.move_to_beginning_of_previous_keyword();
    /// assert_eq!(editor.line(), 0);
    /// assert_eq!(editor.column(), 1);
    /// ```
    pub fn move_to_beginning_of_previous_keyword(&mut self) {
        if let Some(pos) = self.previous_keyword_position() {
            self.set_line(pos.line);
            self.set_column(pos.column);
        }
    }

    /// Returns a position at the beginning of a previous keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// use edit::editor::Position;
    /// let editor = Editor::new("aa \n\
    ///                           bb ", 1, 0).unwrap();
    /// let pos = editor.previous_keyword_position();
    /// assert_eq!(pos, Some(Position::new(0, 0)));
    /// ```
    pub fn previous_keyword_position(&self) -> Option<Position> {
        self.core.previous_keyword_position()
    }

    /// Moves a cursor to the first non-blank character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\n bbb", 1, 3).unwrap();
    /// editor.move_to_beginning_of_non_blank();
    /// assert_eq!(editor.column(), 1);
    /// ```
    pub fn move_to_beginning_of_non_blank(&mut self) {
        if let Some(pos) = self.first_non_blank() {
            self.set_column(pos);
        }
    }

    fn first_non_blank(&self) -> Option<usize> {
        let line = self.current_line_buffer();
        line.iter().position(|ch| !ch.is_whitespace())
    }

    fn current_line_buffer(&self) -> &[char] {
        let buffer = self.buffer();
        let beginning = self.core.offset(self.line(), 0).unwrap();
        let end = self.core
            .offset(self.line(), self.core.current_line_width())
            .unwrap();
        &buffer[beginning..end]
    }

    /// Returns a line of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a\n\
    ///                               b b b b\n\
    ///                               c", 1, 6).unwrap();
    /// assert_eq!(editor.line_buffer(2), Some(&['c'][..]));
    /// ```
    pub fn line_buffer(&self, line: usize) -> Option<&[char]> {
        if self.core.line_count() <= line {
            return None;
        }
        let buffer = self.buffer();
        let beginning = self.core.offset(line, 0).unwrap();
        let end = self.core
            .offset(line, self.core.line_width(line).unwrap())
            .unwrap();
        Some(&buffer[beginning..end])
    }

    /// Moves a cursor to the last non-blank character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("aaa\nbbb ", 1, 1).unwrap();
    /// editor.move_to_end_of_non_blank();
    /// assert_eq!(editor.column(), 3);
    /// ```
    pub fn move_to_end_of_non_blank(&mut self) {
        if let Some(pos) = self.last_non_blank() {
            self.set_column(pos);
        }
    }

    fn last_non_blank(&self) -> Option<usize> {
        let line = self.current_line_buffer();
        line.iter().rposition(|ch| !ch.is_whitespace()).map(
            |i| i + 1,
        )
    }

    /// Moves a cursor to a line.
    ///
    /// If a position is out of the range, nothing happens.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a\n\
    ///                               b b b b\n\
    ///                               c", 1, 6).unwrap();
    /// editor.set_line(2);
    /// assert_eq!(editor.line(), 2);
    /// assert_eq!(editor.column(), 1);
    /// ```
    pub fn set_line(&mut self, n: usize) {
        return self.core.set_line(n);
    }

    /// Moves a cursor to a column.
    ///
    /// If a position is out of the range, nothing happens.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a\n\
    ///                               b b b b\n\
    ///                               c", 1, 1).unwrap();
    /// editor.set_column(5);
    /// assert_eq!(editor.line(), 1);
    /// assert_eq!(editor.column(), 5);
    /// ```
    pub fn set_column(&mut self, n: usize) {
        return self.core.set_column(n);
    }

    /// Moves a cursor to the beginning of the upper line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a\n\
    ///                               b\n\
    ///                               c", 1, 1).unwrap();
    /// editor.move_to_beginning_of_upper_line();
    /// assert_eq!(editor.line(), 2);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_to_beginning_of_upper_line(&mut self) {
        self.set_column(0);
        self.move_down(1);
    }

    /// Moves a cursor to the beginning of the lower line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a\n\
    ///                               b\n\
    ///                               c", 2, 1).unwrap();
    /// editor.move_to_beginning_of_lower_line();
    /// assert_eq!(editor.line(), 1);
    /// assert_eq!(editor.column(), 0);
    /// ```
    pub fn move_to_beginning_of_lower_line(&mut self) {
        self.set_column(0);
        self.move_up(1);
    }

    /// Searches for a character after the cursor in the current line, returning its index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("a b c d\n\
    ///                           x", 0, 3).unwrap();
    /// assert_eq!(editor.search_character('d'), Some(6));
    /// assert_eq!(editor.search_character('x'), None);
    /// ```
    pub fn search_character(&self, ch: char) -> Option<usize> {
        let line = self.current_line_buffer();
        line[self.column()..].iter().position(|&x| x == ch).map(
            |n| {
                n + self.column()
            },
        )
    }

    /// Searches for a character after the cursor in the current line, returning its index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("a b c d\n\
    ///                           x", 0, 3).unwrap();
    /// assert_eq!(editor.rsearch_character('b'), Some(2));
    /// assert_eq!(editor.rsearch_character('x'), None);
    /// ```
    pub fn rsearch_character(&self, ch: char) -> Option<usize> {
        let line = self.current_line_buffer();
        line[..self.column()].iter().rposition(|&x| x == ch).map(
            |n| n,
        )
    }

    /// Join a line and next line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let mut editor = Editor::new("a b c d\n\
    ///                           x", 0, 3).unwrap();
    /// editor.join(0);
    /// let buf: String = editor.buffer().iter().collect();
    /// assert_eq!(buf, "a b c d x");
    /// ```
    pub fn join(&mut self, line: usize) {
        let c = self.line_width(line).expect(
            &format!("line {}: out of range", line),
        );
        self.delete_at(line, c);
        self.insert_at(' ', line, c);
    }

    /// Match parentheses.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// let editor = Editor::new("a ( b ) c", 0, 2).unwrap();
    /// assert_eq!(editor.match_paren(), Some(6));
    ///
    /// let editor = Editor::new("a ( b ) c", 0, 6).unwrap();
    /// assert_eq!(editor.match_paren(), Some(2));
    ///
    /// let editor = Editor::new("a ( b ) c", 0, 0).unwrap();
    /// assert_eq!(editor.match_paren(), None);
    ///
    /// let editor = Editor::new("a ( b ) c", 0, 7).unwrap();
    /// assert_eq!(editor.match_paren(), None);
    ///
    /// let editor = Editor::new("a ( (b) ) c", 0, 8).unwrap();
    /// assert_eq!(editor.match_paren(), Some(2));
    /// ```
    pub fn match_paren(&self) -> Option<usize> {
        self.match_pair(Paren {
            open: '(',
            close: ')',
        })
    }

    /// Matches a pair of parentheses.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Editor;
    /// use edit::editor::Paren;
    /// let editor = Editor::new("a { b } c", 0, 2).unwrap();
    /// assert_eq!(editor.match_pair(Paren {open: '{', close: '}'}), Some(6));
    ///
    /// let editor = Editor::new(" [1, 3) ", 0, 1).unwrap();
    /// assert_eq!(editor.match_pair(Paren {open: '[', close: ')'}), Some(6));
    /// ```
    pub fn match_pair(&self, p: Paren) -> Option<usize> {
        let n = self.core.current_offset();
        let mut level: usize = 0;
        let x = self.buffer()[n];
        if x == p.open {
            self.buffer()[n + 1..]
                .iter()
                .position(|&c| {
                    if c == p.open {
                        level += 1;
                        return false;
                    }
                    if c != p.close {
                        return false;
                    }
                    if level == 0 {
                        return true;
                    }
                    level -= 1;
                    false
                })
                .map(|i| i + n + 1)
        } else if x == p.close {
            self.buffer()[..n].iter().rposition(|&c| {
                if c == p.close {
                    level += 1;
                    return false;
                }
                if c != p.open {
                    return false;
                }
                if level == 0 {
                    return true;
                }
                level -= 1;
                false
            })
        } else {
            None
        }
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

        let buffer = "  aaa \n bbb ";
        let mut editor = Editor::new(buffer, 0, 3).unwrap();
        editor.move_to_beginning_of_next_keyword();
        assert_eq!(editor.line(), 1);
        assert_eq!(editor.column(), 1);
    }

    #[test]
    fn test_move_to_beginning_of_previous_keyword() {
        let buffer = "  aaa  ";
        let mut editor = Editor::new(buffer, 0, 3).unwrap();
        editor.move_to_beginning_of_previous_keyword();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 2);

        let buffer = "  aaa \n    bbb  ";
        let mut editor = Editor::new(buffer, 1, 3).unwrap();
        editor.move_to_beginning_of_previous_keyword();
        assert_eq!(editor.line(), 0);
        assert_eq!(editor.column(), 2);
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
