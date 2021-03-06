use std::cmp::Ordering;
use std::ops::Range;

/// `Position` represents a two-dimensional position which has line and column.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    /// A line number. It is in a range [0, _the number of lines_).
    pub line: usize,
    /// A column number. It is in a range [0, _length at `line`_).
    pub column: usize,
}

impl Position {
    /// Creates a new `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use edit::editor::Position;
    /// let p = Position::new(1, 2);
    /// ```
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        let lc = self.line.partial_cmp(&other.line);
        if lc != Some(Ordering::Equal) {
            return lc;
        }
        self.column.partial_cmp(&other.column)
    }
}

#[derive(PartialEq, Debug)]
pub struct Core {
    buffer: Vec<char>,
    newline_indices: Vec<usize>,
    line: usize,
    column: usize,
}

impl Core {
    pub fn new(buffer: &str, line: usize, column: usize) -> Result<Core, String> {
        let chars: Vec<char> = buffer.chars().collect();
        let mut indices: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| if *ch == '\n' { Some(i) } else { None })
            .collect();
        let char_count = chars.len();
        if indices.last().map(|n| n + 1) != Some(char_count) {
            indices.push(char_count);
        }
        if indices.len() <= line {
            return Err(format!(
                "Line {} is out of range [0, {})",
                line,
                indices.len()
            ));
        }
        let width = indices[line] - if line == 0 { 0 } else { indices[line - 1] + 1 };
        if width < column {
            return Err(format!("Column {} is out of range [0, {}]", column, width));
        }
        Ok(Core {
            buffer: chars,
            newline_indices: indices,
            line,
            column,
        })
    }

    pub fn reset(&mut self, buffer: &str, line: usize, column: usize) {
        let chars: Vec<char> = buffer.chars().collect();
        let mut indices: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| if *ch == '\n' { Some(i) } else { None })
            .collect();
        let char_count = chars.len();
        if indices.last() != Some(&char_count) {
            indices.push(char_count);
        }
        assert!(indices.len() > line);
        let width = indices[line] - if line == 0 { 0 } else { indices[line - 1] + 1 };
        assert!(width >= column);
        self.buffer = chars;
        self.newline_indices = indices;
        self.line = line;
        self.column = column;
    }

    pub fn buffer(&self) -> &[char] {
        &self.buffer
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line_count(&self) -> usize {
        self.newline_indices.len()
    }

    pub fn line_width(&self, n: usize) -> Option<usize> {
        if n >= self.line_count() {
            return None;
        }
        let right = self.newline_indices[n];
        if n == 0 {
            return Some(right);
        }
        let left = self.newline_indices[n - 1] + 1;
        Some(right - left)
    }

    pub fn current_line_width(&self) -> usize {
        self.line_width(self.line).expect(&format!(
            "current_line_width: unexpected error (line: {})",
            self.line
        ))
    }

    pub fn offset(&self, line: usize, column: usize) -> Option<usize> {
        if line >= self.line_count() || self.line_width(line).unwrap() < column {
            return None;
        }
        if line == 0 {
            return Some(column);
        }
        let line_offset = self.newline_indices[line - 1] + 1;
        Some(line_offset + column)
    }

    pub fn current_offset(&self) -> usize {
        self.offset(self.line, self.column).expect(&format!(
            "current_offset: unexpected error (line: {}, column: {})",
            self.line,
            self.column,
        ))
    }

    pub fn set_column(&mut self, n: usize) {
        if n <= self.current_line_width() {
            self.column = n;
        }
    }

    pub fn set_line(&mut self, n: usize) {
        if n < self.line_count() {
            self.line = n;
        }
        let width = self.current_line_width();
        if width < self.column {
            self.column = width;
        }
    }

    pub fn move_left(&mut self, n: usize) {
        if self.column < n {
            self.column = 0;
            return;
        }
        self.column -= n;
    }

    pub fn move_right(&mut self, n: usize) {
        let width = self.current_line_width();
        if self.column + n >= width {
            self.column = width;
            return;
        }
        self.column += n;
    }

    pub fn move_up(&mut self, n: usize) {
        if self.line < n {
            self.line = 0;
        } else {
            self.line -= n;
        }
        let width = self.current_line_width();
        if width < self.column {
            self.column = width;
        }
    }

    pub fn move_down(&mut self, n: usize) {
        if self.line + n >= self.line_count() {
            self.line = self.line_count() - 1;
        } else {
            self.line += n;
        }
        let width = self.current_line_width();
        if width < self.column {
            self.column = width;
        }
    }

    pub fn insert_at(&mut self, ch: char, line: usize, column: usize) {
        let offset = self.offset(line, column);
        if offset.is_none() {
            return;
        }
        let i = offset.unwrap();
        let current_offset = self.current_offset();
        self.buffer.insert(i, ch);
        for x in self.newline_indices[line..].iter_mut() {
            *x += 1
        }
        if ch == '\n' {
            self.newline_indices.insert(line, i);
        }
        if ch == '\n' && i <= current_offset {
            if self.line == line {
                self.column = current_offset - i;
            }
            self.line += 1;
            return;
        }
        if line == self.line && column <= self.column {
            self.column += 1;
        }
    }

    pub fn insert_string_at(&mut self, s: &str, line: usize, column: usize) {
        for ch in s.chars().rev() {
            self.insert_at(ch, line, column)
        }
    }

    pub fn delete_at(&mut self, line: usize, column: usize) {
        let line_width = self.line_width(line);
        if line_width.is_none() {
            return;
        }
        let line_width = line_width.unwrap();
        if self.line_count() <= line || line_width < column {
            return;
        }
        let current_offset = self.current_offset();
        let width = self.line_width(line).expect(&format!("width: {}", line));
        let offset = self.offset(line, column).expect(&format!(
            "offset: {} {}",
            line,
            column
        ));
        let ch = self.buffer.remove(offset);
        if ch == '\n' {
            self.newline_indices.remove(line);
        }
        for x in self.newline_indices[line..].iter_mut() {
            *x -= 1
        }
        if ch == '\n' && offset < current_offset {
            self.line -= 1;
            if self.line == line {
                self.column = width + current_offset - offset - 1;
            }
            return;
        }
        if line != self.line {
            return;
        }
        if column < self.column {
            self.column -= 1;
        }
    }

    pub fn delete_range(&mut self, range: Range<Position>) {
        let start = self.offset(range.start.line, range.start.column).expect(
            &format!(
                "out of range: {:?}",
                range
                    .start
            ),
        );
        let n = self.offset(range.end.line, range.end.column).expect(
            &format!(
                "out of range: {:?}",
                range.end
            ),
        ) - start;
        for _ in 0..n {
            self.delete_at(range.start.line, range.start.column)
        }
    }

    pub fn next_position(&self, f: fn(char) -> bool) -> Option<Position> {
        let off = self.current_offset();
        let indices = &self.newline_indices[self.line..];
        let mut it = self.buffer[off..].iter();
        let p = it.position(|&ch| !f(ch));
        if p.is_none() {
            return None;
        }
        let p = p.unwrap();
        it.position(|&ch| f(ch)).map(|n| n + off + p).map(|n| {
            let i = indices.iter().position(|&x| n < x).expect(
                "next_position: unexpected error",
            ) + self.line;
            if i == self.line {
                return Position::new(i, self.column + n - off + 1);
            }
            Position::new(i, n - self.newline_indices[i - 1])
        })
    }

    pub fn previous_position(&self, f: fn(char) -> bool) -> Option<Position> {
        let off = self.current_offset();
        let indices = &self.newline_indices[..self.line];
        let mut it = self.buffer[..off].iter();
        if it.rposition(|&ch| f(ch)).is_none() {
            return None;
        }
        it.rposition(|&ch| !f(ch))
            .map(|n| n + 1)
            .map(|n| {
                let i = indices.iter().rposition(|&x| n > x);
                if i == None {
                    return Position::new(0, n);
                }
                let i = i.unwrap();
                Position::new(i + 1, n - self.newline_indices[i] - 1)
            })
            .or(Some(Position::new(0, 0)))
    }

    pub fn next_keyword_position(&self) -> Option<Position> {
        self.next_position(char::is_alphanumeric)
    }

    pub fn previous_keyword_position(&self) -> Option<Position> {
        self.previous_position(char::is_alphanumeric)
    }

    pub fn next_symbol_position(&self) -> Option<Position> {
        self.next_position(char::is_symbol)
    }

    pub fn previous_symbol_position(&self) -> Option<Position> {
        self.previous_position(char::is_symbol)
    }

    pub fn next_end_position(&self, f: fn(char) -> bool) -> Option<Position> {
        let off = self.current_offset();
        let indices = &self.newline_indices[self.line..];
        let mut it = self.buffer[off..].iter();
        let p = it.position(|&ch| f(ch));
        if p.is_none() {
            return None;
        }
        let p = p.unwrap();
        it.position(|&ch| !f(ch)).map(|n| n + off + p - 1).map(|n| {
            let i = indices.iter().position(|&x| n < x).expect(
                "next_end_position: unexpected error",
            ) + self.line;
            if i == self.line {
                return Position::new(i, self.column + n - off + 1);
            }
            Position::new(i, n - self.newline_indices[i - 1])
        })
    }

    pub fn previous_end_position(&self, f: fn(char) -> bool) -> Option<Position> {
        let off = self.current_offset();
        let indices = &self.newline_indices[..self.line];
        let mut it = self.buffer[..off].iter();
        if it.rposition(|&ch| !f(ch)).is_none() {
            return None;
        }
        it.rposition(|&ch| f(ch))
            .map(|n| {
                let i = indices.iter().rposition(|&x| n > x);
                if i == None {
                    return Position::new(0, n);
                }
                let i = i.unwrap();
                Position::new(i, n - self.newline_indices[i])
            })
            .or(Some(Position::new(0, 0)))
    }

    pub fn next_keyword_end_position(&self) -> Option<Position> {
        self.next_end_position(char::is_alphanumeric)
    }

    pub fn previous_keyword_end_position(&self) -> Option<Position> {
        self.previous_end_position(char::is_alphanumeric)
    }

    pub fn next_symbol_end_position(&self) -> Option<Position> {
        self.next_end_position(char::is_symbol)
    }

    pub fn previous_symbol_end_position(&self) -> Option<Position> {
        self.previous_end_position(char::is_symbol)
    }

    pub fn after_position(&self, f: fn(char) -> bool) -> Option<Position> {
        self.next_end_position(f).map(|p| if p.column <
            self.line_width(p.line)
                .unwrap()
        {
            Position::new(p.line, p.column + 1)
        } else {
            Position::new(p.line + 1, 0)
        })
    }

    pub fn before_position(&self, f: fn(char) -> bool) -> Option<Position> {
        self.previous_position(f).and_then(|p| if p.column == 0 {
            if p.line == 0 {
                None
            } else {
                Some(Position::new(p.line - 1, 0))
            }
        } else {
            Some(Position::new(p.line, p.column - 1))
        })
    }

    pub fn after_keyword_position(&self) -> Option<Position> {
        self.after_position(char::is_alphanumeric)
    }

    pub fn before_keyword_position(&self) -> Option<Position> {
        self.before_position(char::is_alphanumeric)
    }

    pub fn after_symbol_position(&self) -> Option<Position> {
        self.after_position(char::is_symbol)
    }

    pub fn before_symbol_position(&self) -> Option<Position> {
        self.before_position(char::is_symbol)
    }
}

impl Clone for Core {
    fn clone(&self) -> Core {
        Core {
            buffer: self.buffer.clone(),
            line: self.line,
            column: self.column,
            newline_indices: self.newline_indices.clone(),
        }
    }
}

trait Character {
    fn is_symbol(self) -> bool;
}

impl Character for char {
    fn is_symbol(self) -> bool {
        !(self.is_alphanumeric() || self.is_whitespace())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(buffer, 10, 10);
        assert!(editor.is_err());
    }

    #[test]
    fn test_line_count() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.line_count(), 2);

        let editor = Core::new("", 0, 0).unwrap();
        assert_eq!(editor.line_count(), 1);

        let editor = Core::new("aaa bbb", 0, 0).unwrap();
        assert_eq!(editor.line_count(), 1);
    }

    #[test]
    fn test_line_width() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(13));
        assert_eq!(editor.line_width(1), Some(13));
        assert_eq!(editor.line_width(2), None);

        let editor = Core::new("", 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(0));
        assert_eq!(editor.line_width(1), None);

        let editor = Core::new("aaa bbb", 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(7));
        assert_eq!(editor.line_width(1), None);

        let buffer = "世界";
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(2));
        assert_eq!(editor.line_width(1), None);
    }

    #[test]
    fn test_offset() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(1, 1), Some(15));
        assert_eq!(editor.offset(2, 2), None);
        assert_eq!(editor.offset(1, 13), Some(27));
        assert_eq!(editor.offset(1, 14), None);
        assert_eq!(editor.offset(0, 13), Some(13));
        assert_eq!(editor.offset(0, 14), None);

        let editor = Core::new("", 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), None);
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);

        let editor = Core::new("aaa bbb", 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), Some(1));
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);
        assert_eq!(editor.offset(0, 7), Some(7));
        assert_eq!(editor.offset(0, 8), None);

        let buffer = "世界\nabc";
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), Some(1));
        assert_eq!(editor.offset(0, 2), Some(2));
        assert_eq!(editor.offset(1, 0), Some(3));
        assert_eq!(editor.offset(1, 3), Some(6));
        assert_eq!(editor.offset(1, 4), None);
    }

    #[test]
    fn test_move_right() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = Core::new(buffer, 1, 6).unwrap();
        let expected = [7, 8, 9, 10, 11, 12, 13, 13];
        for i in 0..expected.len() {
            editor.move_right(1);
            assert_eq!(editor, Core::new(buffer, 1, expected[i]).unwrap());
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = Core::new(buffer, 1, i).unwrap();
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_right(width + 1);
            assert_eq!(editor, Core::new(buffer, 1, width).unwrap());
        }

        let buffer = "世界\nabc";
        let mut editor = Core::new(buffer, 0, 0).unwrap();
        let expected = [1, 2, 2];
        for i in 0..expected.len() {
            editor.move_right(1);
            assert_eq!(editor, Core::new(buffer, 0, expected[i]).unwrap());
        }
    }

    #[test]
    fn test_move_left() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = Core::new(buffer, 1, 6).unwrap();
        let expected = [5, 4, 3, 2, 1, 0, 0];
        for i in 0..expected.len() {
            editor.move_left(1);
            assert_eq!(editor, Core::new(buffer, 1, expected[i]).unwrap());
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = Core::new(buffer, 1, i).unwrap();
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_left(width + 1);
            assert_eq!(editor, Core::new(buffer, 1, 0).unwrap());
        }

        let buffer = "abc\nHello, 世界\ndef";
        let mut editor = Core::new(buffer, 1, 9).unwrap();
        let expected = [7, 5, 3, 1, 0, 0];
        for i in 0..expected.len() {
            editor.move_left(2);
            assert_eq!(editor, Core::new(buffer, 1, expected[i]).unwrap());
        }
    }

    #[test]
    fn test_move_up() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(buffer, 2, 4).unwrap();
        let expected = [1, 0, 0];
        for i in 0..expected.len() {
            editor.move_up(1);
            assert_eq!(editor, Core::new(buffer, expected[i], 4).unwrap());
        }

        for i in 0..editor.line_count() {
            let mut editor = Core::new(buffer, i, 1).unwrap();
            let count = editor.line_count();
            editor.move_up(count);
            assert_eq!(editor, Core::new(buffer, 0, 1).unwrap());
        }

        let buffer = "aaa\nbbbb";
        let mut editor = Core::new(buffer, 1, 4).unwrap();
        editor.move_up(1);
        assert_eq!(editor, Core::new(buffer, 0, 3).unwrap());
    }

    #[test]
    fn test_move_down() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(buffer, 0, 4).unwrap();
        let expected = [1, 2, 2];
        for i in 0..expected.len() {
            editor.move_down(1);
            assert_eq!(editor, Core::new(buffer, expected[i], 4).unwrap());
        }

        for i in 0..editor.line_count() {
            let mut editor = Core::new(buffer, i, 1).unwrap();
            let count = editor.line_count();
            editor.move_down(count);
            assert_eq!(
                editor,
                Core::new(buffer, buffer.match_indices('\n').count(), 1).unwrap()
            );
        }

        let buffer = "aaaa\nbbb";
        let mut editor = Core::new(buffer, 0, 4).unwrap();
        editor.move_down(1);
        assert_eq!(editor, Core::new(buffer, 1, 3).unwrap());
    }

    #[test]
    fn test_insert_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(buffer, 0, 6).unwrap();
        editor.insert_at('\n', 0, 6);
        assert_eq!(
            editor,
            Core::new("Hello,\n world!\nThe 2nd line.\nAAABBBCCC.", 1, 0).unwrap()
        );
        editor.insert_at('D', 3, 9);
        assert_eq!(
            editor,
            Core::new("Hello,\n world!\nThe 2nd line.\nAAABBBCCCD.", 1, 0).unwrap()
        );
        editor.insert_at('a', 1, 0);
        assert_eq!(
            editor,
            Core::new("Hello,\na world!\nThe 2nd line.\nAAABBBCCCD.", 1, 1).unwrap()
        );

        let buffer = "aaa";
        let mut editor = Core::new(buffer, 0, 0).unwrap();
        editor.insert_at('a', 10, 10);
        assert_eq!(editor, Core::new(buffer, 0, 0).unwrap());

        let buffer = "💖a";
        let mut editor = Core::new(buffer, 0, 0).unwrap();
        editor.insert_at('💖', 0, 2);
        let want = "💖a💖";
        assert_eq!(editor, Core::new(want, 0, 0).unwrap());
    }

    #[test]
    fn test_insert_string_at() {
        let buffer = "aaa ccc ddd";
        let mut editor = Core::new(buffer, 0, 7).unwrap();
        editor.insert_string_at("bbb ", 0, 4);
        assert_eq!(editor, Core::new("aaa bbb ccc ddd", 0, 11).unwrap());
    }

    #[test]
    fn test_delete_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(buffer, 0, 6).unwrap();
        editor.delete_at(0, 6);
        assert_eq!(
            editor,
            Core::new("Hello,world!\nThe 2nd line.\nAAABBBCCC.", 0, 6).unwrap()
        );
        editor.delete_at(0, 12);
        assert_eq!(
            editor,
            Core::new("Hello,world!The 2nd line.\nAAABBBCCC.", 0, 6).unwrap()
        );

        let mut editor = Core::new("abc\ndef", 0, 3).unwrap();
        editor.delete_at(0, 2);
        assert_eq!(editor, Core::new("ab\ndef", 0, 2).unwrap());

        let mut editor = Core::new("abc\ndef", 1, 0).unwrap();
        editor.delete_at(0, 3);
        assert_eq!(editor, Core::new("abcdef", 0, 3).unwrap());
        editor.delete_at(10, 10);
        assert_eq!(editor, Core::new("abcdef", 0, 3).unwrap());
        editor.delete_at(0, 1);
        assert_eq!(editor, Core::new("acdef", 0, 2).unwrap());

        let mut editor = Core::new("abc世界", 0, 3).unwrap();
        editor.delete_at(0, 4);
        assert_eq!(editor, Core::new("abc世", 0, 3).unwrap());
    }

    #[test]
    fn test_delete_range() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(buffer, 0, 6).unwrap();
        editor.delete_range(Position::new(0, 6)..Position::new(1, 5));
        assert_eq!(
            editor,
            Core::new("Hello,nd line.\nAAABBBCCC.", 0, 6).unwrap()
        );
    }

    #[test]
    fn test_next_keyword_position() {
        let buffer = "**\n\
                      a**";
        let editor = Core::new(buffer, 0, 1).unwrap();
        assert_eq!(editor.next_keyword_position(), Some(Position::new(1, 0)));

        let editor = Core::new(buffer, 1, 3).unwrap();
        assert_eq!(editor.next_keyword_position(), None);

        let buffer = " 12wb12 ";
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.next_keyword_position(), Some(Position::new(0, 1)));

        let editor = Core::new(buffer, 0, 1).unwrap();
        assert_eq!(editor.next_keyword_position(), None);
    }

    #[test]
    fn test_previous_keyword_position() {
        let buffer = "**\n\
                      a**";
        let editor = Core::new(buffer, 0, 1).unwrap();
        assert_eq!(editor.previous_keyword_position(), None);

        let editor = Core::new(buffer, 1, 3).unwrap();
        assert_eq!(
            editor.previous_keyword_position(),
            Some(Position::new(1, 0))
        );
    }

    #[test]
    fn test_next_symbol_position() {
        let buffer = "ab\n\
                      *cd";
        let editor = Core::new(buffer, 0, 1).unwrap();

        assert_eq!(editor.next_symbol_position(), Some(Position::new(1, 0)));
    }

    #[test]
    fn test_previous_symbol_position() {
        let buffer = "ab\n\
                      *cd";
        let editor = Core::new(buffer, 0, 1).unwrap();
        assert_eq!(editor.previous_symbol_position(), None);

        let editor = Core::new(buffer, 1, 3).unwrap();
        assert_eq!(editor.previous_symbol_position(), Some(Position::new(1, 0)));
    }

    #[test]
    fn test_after_position() {
        let buffer = "aax\n\
                      aaa";
        let editor = Core::new(buffer, 1, 1).unwrap();
        assert_eq!(editor.after_position(|ch| ch == 'x'), None);

        let editor = Core::new(buffer, 0, 1).unwrap();
        assert_eq!(
            editor.after_position(|ch| ch == 'x'),
            Some(Position::new(0, 3))
        );

        let buffer = "aaa\n\
                      aaa\n";
        let editor = Core::new(buffer, 1, 1).unwrap();
        assert_eq!(editor.after_position(|ch| ch == '\n'), None);
    }

    #[test]
    fn test_before_position() {
        let buffer = "aax\n\
                      aaa";
        let editor = Core::new(buffer, 0, 1).unwrap();
        assert_eq!(editor.before_position(|ch| ch == 'x'), None);

        let editor = Core::new(buffer, 1, 1).unwrap();
        assert_eq!(
            editor.before_position(|ch| ch == 'x'),
            Some(Position::new(0, 1))
        );
    }
}
