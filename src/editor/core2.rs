#![allow(unused)]

use std::fmt;
use std::error;

#[derive(PartialEq, Debug)]
pub struct Core2 {
    buffer: Vec<Vec<char>>,
    line: usize,
    column: usize,
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
        if self.column < n {
            self.column = 0;
            return;
        }
        self.column -= n;
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
}
