extern crate test;

#[derive(PartialEq, Debug)]
pub struct Core {
    buffer: String,
    newline_indices: Vec<usize>,
    line: usize,
    column: usize,
}

impl Core {
    pub fn buffer(&self) -> String {
        self.buffer.clone()
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
        };
        Some(
            self.newline_indices[n] - if n == 0 {
                0
            } else {
                self.newline_indices[n - 1] + 1
            },
        )
    }

    pub fn offset(&self, line: usize, column: usize) -> Option<usize> {
        if line >= self.line_count() || self.line_width(line).unwrap() < column {
            return None;
        };
        let line_offset = if line == 0 {
            0
        } else {
            self.newline_indices[(line - 1)] + 1
        };
        Some(line_offset + column)
    }

    pub fn set_column(&mut self, n: usize) {
        if n <= self.line_width(self.line).unwrap() {
            self.column = n;
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
        if self.column + n >= self.line_width(self.line).unwrap() {
            self.column = self.line_width(self.line).unwrap();
            return;
        }
        self.column += n;
    }

    pub fn move_up(&mut self, n: usize) {
        if self.line < n {
            self.line = 0;
            return;
        }
        self.line -= n;
    }

    pub fn move_down(&mut self, n: usize) {
        if self.line + n >= self.line_count() {
            self.line = self.line_count() - 1;
            return;
        }
        self.line += n;
    }

    pub fn insert_at(&mut self, ch: char, line: usize, column: usize) {
        let current_offset = self.offset(self.line, self.column).unwrap();
        let i = self.offset(line, column).unwrap();
        self.buffer.insert(i, ch);
        if ch == '\n' {
            self.newline_indices.insert(line, i);
        }
        for x in self.newline_indices.iter_mut() {
            if *x > i {
                *x += 1
            }
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
        let current_offset = self.offset(self.line, self.column)
            .expect(&format!("current_offset: {} {}", self.line, self.column));
        let width = self.line_width(line).expect(&format!("width: {}", line));
        let offset = self.offset(line, column)
            .expect(&format!("offset: {} {}", line, column));
        let ch = self.buffer.remove(offset);
        if ch == '\n' {
            self.newline_indices.remove(line);
        }
        for x in self.newline_indices.iter_mut() {
            if *x > offset {
                *x -= 1
            }
        }
        if ch == '\n' && offset <= current_offset {
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

pub fn new(buffer: String, line: usize, column: usize) -> Result<Core, String> {
    let mut indices: Vec<usize> = buffer.match_indices('\n').map(|(a, _)| a).collect();
    indices.push(buffer.len());
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
        buffer: buffer.clone(),
        newline_indices: indices,
        line,
        column,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    #[test]
    fn test_build_editor() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = new(String::from(buffer), 10, 10);
        assert!(editor.is_err());
    }

    #[test]
    fn test_line_count() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = new(String::from(buffer), 0, 0).unwrap();
        assert_eq!(editor.line_count(), 2);

        let editor = new(String::new(), 0, 0).unwrap();
        assert_eq!(editor.line_count(), 1);

        let editor = new(String::from("aaa bbb"), 0, 0).unwrap();
        assert_eq!(editor.line_count(), 1);
    }

    #[test]
    fn test_line_width() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = new(String::from(buffer), 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(13));
        assert_eq!(editor.line_width(1), Some(13));
        assert_eq!(editor.line_width(2), None);

        let editor = new(String::new(), 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(0));
        assert_eq!(editor.line_width(1), None);

        let editor = new(String::from("aaa bbb"), 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(7));
        assert_eq!(editor.line_width(1), None);
    }

    #[test]
    fn test_offset() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = new(String::from(buffer), 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(1, 1), Some(15));
        assert_eq!(editor.offset(2, 2), None);
        assert_eq!(editor.offset(1, 13), Some(27));
        assert_eq!(editor.offset(1, 14), None);
        assert_eq!(editor.offset(0, 13), Some(13));
        assert_eq!(editor.offset(0, 14), None);

        let editor = new(String::new(), 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), None);
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);

        let editor = new(String::from("aaa bbb"), 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), Some(1));
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);
        assert_eq!(editor.offset(0, 7), Some(7));
        assert_eq!(editor.offset(0, 8), None);
    }

    #[test]
    fn test_move_right() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = new(String::from(buffer), 1, 6).unwrap();
        let expected = [7, 8, 9, 10, 11, 12, 13, 13];
        for i in 0..expected.len() {
            editor.move_right(1);
            assert_eq!(editor, new(String::from(buffer), 1, expected[i]).unwrap());
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = new(String::from(buffer), 1, i).unwrap();
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_right(width + 1);
            assert_eq!(editor, new(String::from(buffer), 1, width).unwrap());
        }
    }

    #[test]
    fn test_move_left() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = new(String::from(buffer), 1, 6).unwrap();
        let expected = [5, 4, 3, 2, 1, 0, 0];
        for i in 0..expected.len() {
            editor.move_left(1);
            assert_eq!(editor, new(String::from(buffer), 1, expected[i]).unwrap());
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = new(String::from(buffer), 1, i).unwrap();
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_left(width + 1);
            assert_eq!(editor, new(String::from(buffer), 1, 0).unwrap());
        }
    }

    #[test]
    fn test_move_up() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = new(String::from(buffer), 2, 4).unwrap();
        let expected = [1, 0, 0];
        for i in 0..expected.len() {
            editor.move_up(1);
            assert_eq!(editor, new(String::from(buffer), expected[i], 4).unwrap());
        }

        for i in 0..editor.line_count() {
            let mut editor = new(String::from(buffer), i, 1).unwrap();
            let count = editor.line_count();
            editor.move_up(count);
            assert_eq!(editor, new(String::from(buffer), 0, 1).unwrap());
        }
    }

    #[test]
    fn test_move_down() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = new(String::from(buffer), 0, 4).unwrap();
        let expected = [1, 2, 2];
        for i in 0..expected.len() {
            editor.move_down(1);
            assert_eq!(editor, new(String::from(buffer), expected[i], 4).unwrap());
        }

        for i in 0..editor.line_count() {
            let mut editor = new(String::from(buffer), i, 1).unwrap();
            let count = editor.line_count();
            editor.move_down(count);
            assert_eq!(
                editor,
                new(String::from(buffer), buffer.match_indices('\n').count(), 1,).unwrap()
            );
        }
    }

    #[test]
    fn test_insert_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = new(String::from(buffer), 0, 6).unwrap();
        editor.insert_at('\n', 0, 6);
        assert_eq!(
            editor,
            new(
                String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCC."),
                1,
                0,
            ).unwrap()
        );
        editor.insert_at('D', 3, 9);
        assert_eq!(
            editor,
            new(
                String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCCD."),
                1,
                0,
            ).unwrap()
        );
        editor.insert_at('a', 1, 0);
        assert_eq!(
            editor,
            new(
                String::from("Hello,\na world!\nThe 2nd line.\nAAABBBCCCD."),
                1,
                1,
            ).unwrap()
        );
    }

    #[test]
    fn test_insert_string_at() {
        let buffer = "aaa ccc ddd";
        let mut editor = new(String::from(buffer), 0, 7).unwrap();
        editor.insert_string_at("bbb ", 0, 4);
        assert_eq!(
            editor,
            new(String::from("aaa bbb ccc ddd"), 0, 11,).unwrap()
        );
    }

    #[test]
    fn test_delete_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = new(String::from(buffer), 0, 6).unwrap();
        editor.delete_at(0, 6);
        assert_eq!(
            editor,
            new(
                String::from("Hello,world!\nThe 2nd line.\nAAABBBCCC."),
                0,
                6,
            ).unwrap()
        );
        editor.delete_at(0, 12);
        assert_eq!(
            editor,
            new(String::from("Hello,world!The 2nd line.\nAAABBBCCC."), 0, 6).unwrap()
        );

        let mut editor = new(String::from("abc\ndef"), 0, 3).unwrap();
        editor.delete_at(0, 2);
        assert_eq!(editor, new(String::from("ab\ndef"), 0, 2).unwrap());

        let mut editor = new(String::from("abc\ndef"), 1, 0).unwrap();
        editor.delete_at(0, 3);
        assert_eq!(editor, new(String::from("abcdef"), 0, 3).unwrap());
        editor.delete_at(10, 10);
        assert_eq!(editor, new(String::from("abcdef"), 0, 3).unwrap());
        editor.delete_at(0, 1);
        assert_eq!(editor, new(String::from("acdef"), 0, 2).unwrap());
    }

    #[bench]
    fn bench_move_right(b: &mut Bencher) {
        let mut editor = new(String::from("abcdef").repeat(10000), 0, 0).unwrap();
        b.iter(|| editor.move_right(10));
    }

    #[bench]
    fn bench_insert_at(b: &mut Bencher) {
        let buffer = String::from("abcdef").repeat(10000);
        let editor = new(buffer.clone(), 0, 0).unwrap();
        b.iter(|| {
            let mut ed = editor.clone();
            ed.insert_at('x', 0, 500)
        });
    }

    fn bench_insert_string_at_n(b: &mut Bencher, s: &str, n: usize) {
        let buffer = String::from("abcdef").repeat(10000);
        let editor = new(buffer, 0, 0).unwrap();
        b.iter(|| {
            let mut ed = editor.clone();
            ed.insert_string_at(String::from(s).repeat(n).as_str(), 0, 500)
        });
    }

    #[bench]
    fn bench_insert_string_at_1(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 1);
    }

    #[bench]
    fn bench_insert_string_at_2(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 2);
    }

    #[bench]
    fn bench_insert_string_at_4(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 4);
    }

    #[bench]
    fn bench_insert_string_at_8(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 8);
    }

    #[bench]
    fn bench_insert_string_at_16(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 16);
    }

    #[bench]
    fn bench_insert_string_at_256(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 256);
    }

    #[bench]
    fn bench_insert_string_at_1024(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 1024);
    }


    #[bench]
    fn bench_insert_string_at_1_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 1);
    }

    #[bench]
    fn bench_insert_string_at_2_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 2);
    }

    #[bench]
    fn bench_insert_string_at_4_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 4);
    }

    #[bench]
    fn bench_insert_string_at_8_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 8);
    }

    #[bench]
    fn bench_insert_string_at_16_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 16);
    }

    #[bench]
    fn bench_buffer(b: &mut Bencher) {
        let buffer = String::from("abcdef").repeat(10000);
        b.iter(|| {
            let mut buf = buffer.clone();
            buf.insert(3, 'x')
        });
    }
}
