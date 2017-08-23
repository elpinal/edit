#![allow(unused)]

#[derive(PartialEq, Debug)]
pub struct Core2 {
    buffer: Vec<Vec<char>>,
    line: usize,
    column: usize,
}

impl Core2 {
    pub fn new(buffer: &str, line: usize, column: usize) -> Result<Core2, String> {
        let buf: Vec<Vec<char>> = buffer.lines().map(|l| l.chars().collect()).collect();

        if buf.len() <= line {
            return Err(format!("Line {} is out of range [0, {})", line, buf.len()));
        }
        let width = buf[line].len();
        if width < column {
            return Err(format!("Column {} is out of range [0, {}]", column, width));
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

    pub fn line_width(&self, n: usize) -> Option<usize> {
        self.buffer.get(n).map(|l| l.len())
    }

    pub fn offset(&self, line: usize, column: usize) -> Option<usize> {
        if let Some(w) = self.line_width(line) {
            if w < column {
                return None;
            }
        } else {
            return None;
        }
        Some(
            self.buffer[..line]
                .iter()
                .map(|l| l.len() + 1)
                .sum::<usize>() + column,
        )
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
        assert_eq!(got, Some(5));

        let got = editor.line_width(1);
        assert_eq!(got, None);
    }

    #[test]
    fn test_offset() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.offset(0, 0);
        assert_eq!(got, Some(0));

        let got = editor.offset(0, 5);
        assert_eq!(got, Some(5));

        let got = editor.offset(0, 6);
        assert_eq!(got, None);

        let buffer = "aa aa\n\
                      bb bb";
        let editor = Core2::new(buffer, 0, 0).unwrap();
        let got = editor.offset(1, 0);
        assert_eq!(got, Some(6));

        let got = editor.offset(1, 5);
        assert_eq!(got, Some(11));

        let got = editor.offset(1, 6);
        assert_eq!(got, None);
    }
}
