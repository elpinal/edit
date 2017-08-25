#![allow(unused)]

use std::fmt;
use std::error;

use editor::core::Position;
use editor::iterator2d::Iterator2d;

#[derive(PartialEq, Debug)]
pub struct Core2 {
    buffer: Vec<Vec<char>>,
    line: usize,
    column: usize,

    virtual_column: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub enum PositionError {
    Line(usize),
    Column(usize),
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PositionError::Line(n) => write!(f, "line {} is out of range", n),
            PositionError::Column(n) => write!(f, "column {} is out of range", n),
        }
    }
}

impl error::Error for PositionError {
    fn description(&self) -> &str {
        match *self {
            PositionError::Line(..) => "line is out of range",
            PositionError::Column(..) => "column is out of range",
        }
    }
}

impl Core2 {
    pub fn new(buffer: &str, line: usize, column: usize) -> Result<Core2, PositionError> {
        let buf: Vec<Vec<char>> = buffer.lines().map(|l| l.chars().collect()).collect();

        if buf.len() <= line {
            return Err(PositionError::Line(line));
        }
        let width = buf[line].len();
        if width < column {
            return Err(PositionError::Column(column));
        }
        Ok(Core2 {
            buffer: buf,
            line,
            column,
            virtual_column: None,
        })
    }

    pub fn buffer(&self) -> &[Vec<char>] {
        &self.buffer
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line_count(&self) -> usize {
        self.buffer.len()
    }

    pub fn line_width(&self, line: usize) -> Result<usize, PositionError> {
        self.buffer.get(line).map(|l| l.len()).ok_or(
            PositionError::Line(
                line,
            ),
        )
    }

    pub fn current_line_width(&self) -> usize {
        self.line_width(self.line).unwrap()
    }

    pub fn offset(&self, line: usize, column: usize) -> Result<usize, PositionError> {
        let w = self.line_width(line)?;
        if w < column {
            return Err(PositionError::Column(column));
        }
        Ok(
            self.buffer[..line]
                .iter()
                .map(|l| l.len() + 1)
                .sum::<usize>() + column,
        )
    }

    pub fn set_position(&mut self, line: usize, column: usize) {
        let n = self.line_count();
        assert!(line < n, "line {} is out of bounds of {:?}", line, 0..n);
        let w = self.line_width(line).unwrap();
        assert!(
            column <= w,
            "column {} is out of bounds of {:?}",
            column,
            0..w + 1
        );
        self.line = line;
        self.column = column;
    }

    pub fn set_column(&mut self, column: usize) {
        assert!(column <= self.current_line_width());
        self.column = column;
    }

    pub fn move_left(&mut self, n: usize) {
        let c = self.column;
        if self.column < n {
            self.column = 0;
        } else {
            self.column -= n;
        }
        if self.column != c {
            self.virtual_column = None;
        }
    }

    pub fn move_right(&mut self, n: usize) {
        let w = self.current_line_width();
        if self.column + n >= w {
            self.column = w;
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
        if let Some(vc) = self.virtual_column {
            self.column = vc;
        }
        let w = self.current_line_width();
        if w < self.column {
            self.virtual_column = Some(self.column);
            self.column = w;
        }
    }

    pub fn move_down(&mut self, n: usize) {
        let lc = self.line_count();
        if self.line + n >= lc {
            self.line = lc - 1;
        } else {
            self.line += n;
        }
        if let Some(vc) = self.virtual_column {
            self.column = vc;
        }
        let w = self.current_line_width();
        if w < self.column {
            self.virtual_column = Some(self.column);
            self.column = w;
        }
    }

    pub fn insert_at(&mut self, ch: char, line: usize, column: usize) -> Result<(), PositionError> {
        self.buffer
            .get_mut(line)
            .ok_or(PositionError::Line(line))
            .and_then(|l| if column <= l.len() {
                Ok(l.insert(column, ch))
            } else {
                Err(PositionError::Column(column))
            })?;
        if self.line == line && column <= self.column {
            self.column += 1;
        }
        Ok(())
    }

    pub fn delete_at(&mut self, line: usize, column: usize) -> Result<(), PositionError> {
        self.buffer
            .get_mut(line)
            .ok_or(PositionError::Line(line))
            .and_then(|l| if column <= l.len() {
                Ok(l.remove(column))
            } else {
                Err(PositionError::Column(column))
            })?;
        if self.line == line && column < self.column {
            self.column -= 1;
        }
        Ok(())
    }

    pub fn next_position(&self, f: fn(char) -> bool) -> Option<Position> {
        let mut it = Iterator2d::new(self.buffer());
        it.skip(self.line, self.column);
        it.position(|&ch| !f(ch))
            .and(it.position(|&ch| f(ch)))
            .map(|(x, y)| Position::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0);
        assert!(editor.is_ok());

        let editor = Core2::new(buffer, 0, 6);
        assert!(editor.is_err());
    }

    #[test]
    fn test_buffer() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.buffer();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0], vec!['a', 'a', ' ', 'a', 'a']);
    }

    #[test]
    fn test_line() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.line();
        assert_eq!(got, 0);
    }

    #[test]
    fn test_column() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.column();
        assert_eq!(got, 0);
    }

    #[test]
    fn test_line_count() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.line_count();
        assert_eq!(got, 1);
    }

    #[test]
    fn test_line_width() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.line_width(0);
        assert_eq!(got, Ok(5));

        let got = editor.line_width(1);
        assert_eq!(got.ok(), None);
    }

    #[test]
    fn test_offset() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.offset(0, 0);
        assert_eq!(got, Ok(0));

        let got = editor.offset(0, 5);
        assert_eq!(got, Ok(5));

        let got = editor.offset(0, 6);
        assert_eq!(got.ok(), None);

        let buffer = "aa aa\n\
                      bb bb";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.offset(1, 0);
        assert_eq!(got, Ok(6));

        let got = editor.offset(1, 5);
        assert_eq!(got, Ok(11));

        let got = editor.offset(1, 6);
        assert_eq!(got.ok(), None);
    }

    #[test]
    fn test_set_position() {
        let buffer = "aa aa\n\
                      bb bb";
        let mut editor = Core2::new(buffer, 0, 0).unwrap();
        editor.set_position(0, 0);
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 0);

        editor.set_position(1, 1);
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 1);
    }

    #[test]
    fn test_set_column() {
        let buffer = "aa aa";
        let mut editor = Core2::new(buffer, 0, 0).unwrap();
        editor.set_column(2);
        assert_eq!(editor.column, 2);

        editor.set_column(5);
        assert_eq!(editor.column, 5);
    }

    #[test]
    fn test_move_around() {
        let buffer = "aa aa\n\
                      bb bb\n\
                      cc\n\
                      dd d\n\
                      e\n\
                      \n\
                      gg";
        let mut editor = Core2::new(buffer, 0, 0).unwrap();

        editor.move_right(1);
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 1);

        editor.move_right(2);
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 3);

        editor.move_left(1);
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 2);

        editor.move_down(1);
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 2);

        editor.move_down(5);
        assert_eq!(editor.line, 6);
        assert_eq!(editor.column, 2);

        editor.move_up(1);
        assert_eq!(editor.line, 5);
        assert_eq!(editor.column, 0);

        editor.move_up(1);
        assert_eq!(editor.line, 4);
        assert_eq!(editor.column, 1);

        editor.move_up(1);
        assert_eq!(editor.line, 3);
        assert_eq!(editor.column, 2);

        editor.move_left(1);
        assert_eq!(editor.line, 3);
        assert_eq!(editor.column, 1);

        editor.move_up(1);
        assert_eq!(editor.line, 2);
        assert_eq!(editor.column, 1);
    }

    fn str_to_lines(s: &str) -> Vec<Vec<char>> {
        s.lines().map(|l| l.chars().collect()).collect()
    }

    #[test]
    fn test_insert_at() {
        let buffer = "aa aa\n\
                      bb bb";
        let mut editor = Core2::new(buffer, 0, 0).unwrap();
        editor.insert_at('b', 0, 5);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aab\n\
                 bb bb",
            )
        );
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 0);

        let mut editor = Core2::new(buffer, 0, 5).unwrap();
        editor.insert_at('b', 0, 2);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aab aa\n\
                 bb bb",
            )
        );
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 6);

        let mut editor = Core2::new(buffer, 0, 5).unwrap();
        editor.insert_at('b', 0, 5);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aab\n\
                 bb bb",
            )
        );
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 6);

        let mut editor = Core2::new(buffer, 0, 5).unwrap();
        editor.insert_at('b', 1, 5);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aa\n\
                 bb bbb",
            )
        );
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 5);

        let mut editor = Core2::new(buffer, 1, 4).unwrap();
        editor.insert_at('b', 1, 5);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aa\n\
                 bb bbb",
            )
        );
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 4);

        let mut editor = Core2::new(buffer, 1, 4).unwrap();
        editor.insert_at('b', 1, 2);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aa\n\
                 bbb bb",
            )
        );
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 5);

        let mut editor = Core2::new(buffer, 1, 4).unwrap();
        editor.insert_at('b', 1, 4);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aa\n\
                 bb bbb",
            )
        );
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 5);

        let mut editor = Core2::new(buffer, 1, 4).unwrap();
        editor.insert_at('b', 0, 4);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aba\n\
                 bb bb",
            )
        );
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 4);
    }

    #[test]
    fn test_delete_at() {
        let buffer = "aa aa\n\
                      bb bb";
        let mut editor = Core2::new(buffer, 0, 0).unwrap();
        editor.delete_at(1, 2);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aa\n\
                 bbbb",
            )
        );
        assert_eq!(editor.line, 0);
        assert_eq!(editor.column, 0);

        let mut editor = Core2::new(buffer, 1, 0).unwrap();
        editor.delete_at(0, 2);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aaaa\n\
                 bb bb",
            )
        );
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 0);

        let mut editor = Core2::new(buffer, 1, 0).unwrap();
        editor.delete_at(1, 0);
        assert_eq!(
            editor.buffer,
            str_to_lines(
                "aa aa\n\
                 b bb",
            )
        );
        assert_eq!(editor.line, 1);
        assert_eq!(editor.column, 0);
    }

    #[test]
    fn test_next_position() {
        let buffer = "**\n\
                      a**";
        let editor = Core2::new(buffer, 0, 1).unwrap();
        assert_eq!(
            editor.next_position(|ch| ch == 'a'),
            Some(Position::new(1, 0))
        );

        let editor = Core2::new(buffer, 1, 1).unwrap();
        assert_eq!(editor.next_position(|ch| ch == 'a'), None);

        let editor = Core2::new(buffer, 0, 1).unwrap();
        assert_eq!(
            editor.next_position(|ch| ch == '*'),
            Some(Position::new(1, 1))
        );

        assert_eq!(editor.next_position(|ch| ch == '-'), None);
    }
}
