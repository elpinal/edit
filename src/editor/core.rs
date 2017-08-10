extern crate test;

#[derive(PartialEq, Debug)]
pub struct Core {
    buffer: String,
    newline_indices: Vec<usize>,
    line: usize,
    column: usize,
}

impl Core {
    pub fn new(buffer: String, line: usize, column: usize) -> Result<Core, String> {
        let mut indices: Vec<usize> = buffer
            .chars()
            .enumerate()
            .filter_map(|(i, ch)| if ch == '\n' { Some(i) } else { None })
            .collect();
        let char_count = buffer.chars().count();
        if indices.last() != Some(&char_count) {
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
            buffer: buffer.clone(),
            newline_indices: indices,
            line,
            column,
        })
    }

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
        let right = self.newline_indices[n];
        let left = if n == 0 {
            0
        } else {
            self.newline_indices[n - 1] + 1
        };
        Some(right.checked_sub(left).expect(&format!(
            "line_width ({}): unexpected error: {} - {}",
            n,
            right,
            left
        )))
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
        };
        let line_offset = if line == 0 {
            0
        } else {
            self.newline_indices[(line - 1)] + 1
        };
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

    pub fn move_left(&mut self, n: usize) {
        if self.column < n {
            self.column = 0;
            return;
        }
        self.column -= n;
    }

    pub fn move_right(&mut self, n: usize) {
        if self.column + n >= self.current_line_width() {
            self.column = self.current_line_width();
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
        if let Some((n, _)) = self.buffer.char_indices().nth(i) {
            self.buffer.insert(n, ch);
        } else if self.buffer.chars().count() == i {
            let len = self.buffer.len();
            self.buffer.insert(len, ch);
        } else {
            return;
        }
        for x in self.newline_indices.iter_mut() {
            if *x >= i {
                *x += 1
            }
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
        let current_offset = self.offset(self.line, self.column)
            .expect(&format!("current_offset: {} {}", self.line, self.column));
        let width = self.line_width(line).expect(&format!("width: {}", line));
        let offset = self.offset(line, column)
            .expect(&format!("offset: {} {}", line, column));
        let n: usize;
        if let Some((i, _)) = self.buffer.char_indices().nth(offset) {
            n = i;
        } else {
            return;
        }
        let ch = self.buffer.remove(n);
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

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    #[test]
    fn test_build_editor() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(String::from(buffer), 10, 10);
        assert!(editor.is_err());
    }

    #[test]
    fn test_line_count() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(String::from(buffer), 0, 0).unwrap();
        assert_eq!(editor.line_count(), 2);

        let editor = Core::new(String::new(), 0, 0).unwrap();
        assert_eq!(editor.line_count(), 1);

        let editor = Core::new(String::from("aaa bbb"), 0, 0).unwrap();
        assert_eq!(editor.line_count(), 1);
    }

    #[test]
    fn test_line_width() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(String::from(buffer), 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(13));
        assert_eq!(editor.line_width(1), Some(13));
        assert_eq!(editor.line_width(2), None);

        let editor = Core::new(String::new(), 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(0));
        assert_eq!(editor.line_width(1), None);

        let editor = Core::new(String::from("aaa bbb"), 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(7));
        assert_eq!(editor.line_width(1), None);

        let buffer = String::from("世界");
        let editor = Core::new(buffer, 0, 0).unwrap();
        assert_eq!(editor.line_width(0), Some(2));
        assert_eq!(editor.line_width(1), None);
    }

    #[test]
    fn test_offset() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = Core::new(String::from(buffer), 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(1, 1), Some(15));
        assert_eq!(editor.offset(2, 2), None);
        assert_eq!(editor.offset(1, 13), Some(27));
        assert_eq!(editor.offset(1, 14), None);
        assert_eq!(editor.offset(0, 13), Some(13));
        assert_eq!(editor.offset(0, 14), None);

        let editor = Core::new(String::new(), 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), None);
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);

        let editor = Core::new(String::from("aaa bbb"), 0, 0).unwrap();
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), Some(1));
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);
        assert_eq!(editor.offset(0, 7), Some(7));
        assert_eq!(editor.offset(0, 8), None);

        let buffer = String::from("世界\nabc");
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
        let mut editor = Core::new(String::from(buffer), 1, 6).unwrap();
        let expected = [7, 8, 9, 10, 11, 12, 13, 13];
        for i in 0..expected.len() {
            editor.move_right(1);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), 1, expected[i]).unwrap()
            );
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = Core::new(String::from(buffer), 1, i).unwrap();
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_right(width + 1);
            assert_eq!(editor, Core::new(String::from(buffer), 1, width).unwrap());
        }

        let buffer = "世界\nabc";
        let mut editor = Core::new(String::from(buffer), 0, 0).unwrap();
        let expected = [1, 2, 2];
        for i in 0..expected.len() {
            editor.move_right(1);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), 0, expected[i]).unwrap()
            );
        }
    }

    #[test]
    fn test_move_left() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = Core::new(String::from(buffer), 1, 6).unwrap();
        let expected = [5, 4, 3, 2, 1, 0, 0];
        for i in 0..expected.len() {
            editor.move_left(1);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), 1, expected[i]).unwrap()
            );
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = Core::new(String::from(buffer), 1, i).unwrap();
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_left(width + 1);
            assert_eq!(editor, Core::new(String::from(buffer), 1, 0).unwrap());
        }

        let buffer = "abc\nHello, 世界\ndef";
        let mut editor = Core::new(String::from(buffer), 1, 9).unwrap();
        let expected = [7, 5, 3, 1, 0, 0];
        for i in 0..expected.len() {
            editor.move_left(2);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), 1, expected[i]).unwrap()
            );
        }
    }

    #[test]
    fn test_move_up() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(String::from(buffer), 2, 4).unwrap();
        let expected = [1, 0, 0];
        for i in 0..expected.len() {
            editor.move_up(1);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), expected[i], 4).unwrap()
            );
        }

        for i in 0..editor.line_count() {
            let mut editor = Core::new(String::from(buffer), i, 1).unwrap();
            let count = editor.line_count();
            editor.move_up(count);
            assert_eq!(editor, Core::new(String::from(buffer), 0, 1).unwrap());
        }

        let buffer = String::from("aaa\nbbbb");
        let mut editor = Core::new(buffer.clone(), 1, 4).unwrap();
        editor.move_up(1);
        assert_eq!(editor, Core::new(buffer, 0, 3).unwrap());
    }

    #[test]
    fn test_move_down() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(String::from(buffer), 0, 4).unwrap();
        let expected = [1, 2, 2];
        for i in 0..expected.len() {
            editor.move_down(1);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), expected[i], 4).unwrap()
            );
        }

        for i in 0..editor.line_count() {
            let mut editor = Core::new(String::from(buffer), i, 1).unwrap();
            let count = editor.line_count();
            editor.move_down(count);
            assert_eq!(
                editor,
                Core::new(String::from(buffer), buffer.match_indices('\n').count(), 1,).unwrap()
            );
        }

        let buffer = String::from("aaaa\nbbb");
        let mut editor = Core::new(buffer.clone(), 0, 4).unwrap();
        editor.move_down(1);
        assert_eq!(editor, Core::new(buffer, 1, 3).unwrap());
    }

    #[test]
    fn test_insert_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(String::from(buffer), 0, 6).unwrap();
        editor.insert_at('\n', 0, 6);
        assert_eq!(
            editor,
            Core::new(
                String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCC."),
                1,
                0,
            ).unwrap()
        );
        editor.insert_at('D', 3, 9);
        assert_eq!(
            editor,
            Core::new(
                String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCCD."),
                1,
                0,
            ).unwrap()
        );
        editor.insert_at('a', 1, 0);
        assert_eq!(
            editor,
            Core::new(
                String::from("Hello,\na world!\nThe 2nd line.\nAAABBBCCCD."),
                1,
                1,
            ).unwrap()
        );

        let buffer = String::from("aaa");
        let mut editor = Core::new(buffer.clone(), 0, 0).unwrap();
        editor.insert_at('a', 10, 10);
        assert_eq!(editor, Core::new(buffer, 0, 0).unwrap());

        let buffer = String::from("💖a");
        let mut editor = Core::new(buffer, 0, 0).unwrap();
        editor.insert_at('💖', 0, 2);
        let want = String::from("💖a💖");
        assert_eq!(editor, Core::new(want, 0, 0).unwrap());
    }

    #[test]
    fn test_insert_string_at() {
        let buffer = "aaa ccc ddd";
        let mut editor = Core::new(String::from(buffer), 0, 7).unwrap();
        editor.insert_string_at("bbb ", 0, 4);
        assert_eq!(
            editor,
            Core::new(String::from("aaa bbb ccc ddd"), 0, 11,).unwrap()
        );
    }

    #[test]
    fn test_delete_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = Core::new(String::from(buffer), 0, 6).unwrap();
        editor.delete_at(0, 6);
        assert_eq!(
            editor,
            Core::new(
                String::from("Hello,world!\nThe 2nd line.\nAAABBBCCC."),
                0,
                6,
            ).unwrap()
        );
        editor.delete_at(0, 12);
        assert_eq!(
            editor,
            Core::new(String::from("Hello,world!The 2nd line.\nAAABBBCCC."), 0, 6).unwrap()
        );

        let mut editor = Core::new(String::from("abc\ndef"), 0, 3).unwrap();
        editor.delete_at(0, 2);
        assert_eq!(editor, Core::new(String::from("ab\ndef"), 0, 2).unwrap());

        let mut editor = Core::new(String::from("abc\ndef"), 1, 0).unwrap();
        editor.delete_at(0, 3);
        assert_eq!(editor, Core::new(String::from("abcdef"), 0, 3).unwrap());
        editor.delete_at(10, 10);
        assert_eq!(editor, Core::new(String::from("abcdef"), 0, 3).unwrap());
        editor.delete_at(0, 1);
        assert_eq!(editor, Core::new(String::from("acdef"), 0, 2).unwrap());

        let mut editor = Core::new(String::from("abc世界"), 0, 3).unwrap();
        editor.delete_at(0, 4);
        assert_eq!(editor, Core::new(String::from("abc世"), 0, 3).unwrap());
    }

    #[bench]
    fn bench_move_right(b: &mut Bencher) {
        let mut editor = Core::new(String::from("abcdef").repeat(10000), 0, 0).unwrap();
        b.iter(|| editor.move_right(10));
    }

    #[bench]
    fn bench_insert_at(b: &mut Bencher) {
        let buffer = String::from("abcdef").repeat(10000);
        let editor = Core::new(buffer.clone(), 0, 0).unwrap();
        b.iter(|| {
            let mut ed = editor.clone();
            ed.insert_at('x', 0, 500)
        });
    }

    fn bench_insert_string_at_n(b: &mut Bencher, s: &str, n: usize) {
        let buffer = String::from("abcdef").repeat(10000);
        let editor = Core::new(buffer, 0, 0).unwrap();
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
