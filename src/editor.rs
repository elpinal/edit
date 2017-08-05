#[derive(PartialEq, Debug)]
pub struct Editor {
    buffer: String,
    newline_indices: Vec<u32>,
    line: u32,
    column: u32,
}

impl Editor {
    pub fn buffer(&self) -> String {
        self.buffer.clone()
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn line_count(&self) -> u32 {
        self.newline_indices.len() as u32
    }

    pub fn line_width(&self, n: u32) -> Option<u32> {
        if n >= self.line_count() {
            return None;
        };
        Some(
            self.newline_indices[n as usize] - if n == 0 {
                0
            } else {
                self.newline_indices[n as usize - 1] + 1
            },
            )
    }

    pub fn offset(&self, line: u32, column: u32) -> Option<u32> {
        if line >= self.line_count() || self.line_width(line).unwrap() < column {
            return None;
        };
        let line_offset = if line == 0 {
            0
        } else {
            self.newline_indices[(line - 1) as usize] + 1
        };
        Some(line_offset + column)
    }

    pub fn move_left(&mut self, n: u32) {
        if self.column < n {
            self.column = 0;
            return;
        }
        self.column -= n;
    }

    pub fn move_right(&mut self, n: u32) {
        if self.column + n >= self.line_width(self.line).unwrap() {
            self.column = self.line_width(self.line).unwrap();
            return;
        }
        self.column += n;
    }

    pub fn move_up(&mut self, n: u32) {
        if self.line < n {
            self.line = 0;
            return;
        }
        self.line -= n;
    }

    pub fn move_down(&mut self, n: u32) {
        if self.line + n >= self.line_count() {
            self.line = self.line_count() - 1;
            return;
        }
        self.line += n;
    }

    pub fn insert_at(&mut self, ch: char, line: u32, column: u32) {
        let i = self.offset(line, column).unwrap();
        self.buffer.insert(i as usize, ch);
        if ch == '\n' {
            self.newline_indices.insert(line as usize, i);
        }
        for x in self.newline_indices.iter_mut() {
            if *x > i {
                *x += 1
            }
        }
    }

    pub fn delete_at(&mut self, line: u32, column: u32) {
        let line_width = self.line_width(line);
        if line_width.is_none() {
            return;
        }
        let line_width = line_width.unwrap();
        if self.line_count() <= line || line_width < column {
            return;
        }
        let current_offset = self.offset(self.line, self.column).expect(&format!("current_offset: {} {}", self.line, self.column));
        if self.line == line && self.column == line_width {
            self.column -= 1;
        }
        let width = self.line_width(line).expect(&format!("width: {}", line));
        let offset = self.offset(line, column).expect(&format!("offset: {} {}", line, column));
        let ch = self.buffer.remove(offset as usize);
        if ch == '\n' {
            self.newline_indices.remove(line as usize);
            if offset <= current_offset {
                self.line -= 1;
                self.column = width;;
            }
        }
        for x in self.newline_indices.iter_mut() {
            if *x > offset {
                *x -= 1
            }
        }
    }
}

pub fn build_editor(buffer: String, line: u32, column: u32) -> Editor {
    let mut indices: Vec<u32> = buffer.match_indices('\n').map(|(a, _)| a as u32).collect();
    indices.push(buffer.len() as u32);
    Editor {
        buffer: buffer.clone(),
        newline_indices: indices,
        line,
        column,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_count() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = build_editor(String::from(buffer), 0, 0);
        assert_eq!(editor.line_count(), 2);

        let editor = build_editor(String::new(), 0, 0);
        assert_eq!(editor.line_count(), 1);

        let editor = build_editor(String::from("aaa bbb"), 0, 0);
        assert_eq!(editor.line_count(), 1);
    }

    #[test]
    fn test_line_width() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = build_editor(String::from(buffer), 0, 0);
        assert_eq!(editor.line_width(0), Some(13));
        assert_eq!(editor.line_width(1), Some(13));
        assert_eq!(editor.line_width(2), None);

        let editor = build_editor(String::new(), 0, 0);
        assert_eq!(editor.line_width(0), Some(0));
        assert_eq!(editor.line_width(1), None);

        let editor = build_editor(String::from("aaa bbb"), 0, 0);
        assert_eq!(editor.line_width(0), Some(7));
        assert_eq!(editor.line_width(1), None);
    }

    #[test]
    fn test_offset() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let editor = build_editor(String::from(buffer), 0, 0);
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(1, 1), Some(15));
        assert_eq!(editor.offset(2, 2), None);
        assert_eq!(editor.offset(1, 13), Some(27));
        assert_eq!(editor.offset(1, 14), None);
        assert_eq!(editor.offset(0, 13), Some(13));
        assert_eq!(editor.offset(0, 14), None);

        let editor = build_editor(String::new(), 0, 0);
        assert_eq!(editor.offset(0, 0), Some(0));
        assert_eq!(editor.offset(0, 1), None);
        assert_eq!(editor.offset(1, 0), None);
        assert_eq!(editor.offset(1, 1), None);
        assert_eq!(editor.offset(10, 10), None);

        let editor = build_editor(String::from("aaa bbb"), 0, 0);
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
        let mut editor = build_editor(String::from(buffer), 1, 6);
        let expected = [7, 8, 9, 10, 11, 12, 13, 13];
        for i in 0..expected.len() {
            editor.move_right(1);
            assert_eq!(editor, build_editor(String::from(buffer), 1, expected[i]));
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = build_editor(String::from(buffer), 1, i as u32);
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_right(width + 1);
            assert_eq!(
                editor,
                build_editor(String::from(buffer), 1, width)
                );
        }
    }

    #[test]
    fn test_move_left() {
        let buffer = "Hello, world!\nThe 2nd line.";
        let mut editor = build_editor(String::from(buffer), 1, 6);
        let expected = [5, 4, 3, 2, 1, 0, 0];
        for i in 0..expected.len() {
            editor.move_left(1);
            assert_eq!(editor, build_editor(String::from(buffer), 1, expected[i]));
        }

        for i in 0..editor.line_width(editor.line()).unwrap() {
            let mut editor = build_editor(String::from(buffer), 1, i as u32);
            let width = editor.line_width(editor.line()).unwrap();
            editor.move_left(width + 1);
            assert_eq!(editor, build_editor(String::from(buffer), 1, 0));
        }
    }

    #[test]
    fn test_move_up() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = build_editor(String::from(buffer), 2, 4);
        let expected = [1, 0, 0];
        for i in 0..expected.len() {
            editor.move_up(1);
            assert_eq!(editor, build_editor(String::from(buffer), expected[i], 4));
        }

        for i in 0..editor.line_count() {
            let mut editor = build_editor(String::from(buffer), i as u32, 1);
            let count = editor.line_count();
            editor.move_up(count);
            assert_eq!(editor, build_editor(String::from(buffer), 0, 1));
        }
    }

    #[test]
    fn test_move_down() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = build_editor(String::from(buffer), 0, 4);
        let expected = [1, 2, 2];
        for i in 0..expected.len() {
            editor.move_down(1);
            assert_eq!(editor, build_editor(String::from(buffer), expected[i], 4));
        }

        for i in 0..editor.line_count() {
            let mut editor = build_editor(String::from(buffer), i as u32, 1);
            let count = editor.line_count();
            editor.move_down(count);
            assert_eq!(
                editor,
                build_editor(
                    String::from(buffer),
                    buffer.match_indices('\n').count() as u32,
                    1,
                    )
                );
        }
    }

    #[test]
    fn test_insert_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = build_editor(String::from(buffer), 0, 6);
        editor.insert_at('\n', 0, 6);
        assert_eq!(
            editor,
            build_editor(
                String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCC."),
                0,
                6,
                )
            );
        editor.insert_at('D', 3, 9);
        assert_eq!(
            editor,
            build_editor(
                String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCCD."),
                0,
                6,
                )
            );
    }

    #[test]
    fn test_delete_at() {
        let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
        let mut editor = build_editor(String::from(buffer), 0, 6);
        editor.delete_at(0, 6);
        assert_eq!(
            editor,
            build_editor(
                String::from("Hello,world!\nThe 2nd line.\nAAABBBCCC."),
                0,
                6,
                )
            );
        editor.delete_at(0, 12);
        assert_eq!(
            editor,
            build_editor(
                String::from("Hello,world!The 2nd line.\nAAABBBCCC."),
                0,
                6,
                )
            );

        let mut editor = build_editor(String::from("abc\ndef"), 0, 3);
        editor.delete_at(0, 2);
        assert_eq!(
            editor,
            build_editor(
                String::from("ab\ndef"),
                0,
                2,
                )
            );

        let mut editor = build_editor(String::from("abc\ndef"), 1, 0);
        editor.delete_at(0, 3);
        assert_eq!(
            editor,
            build_editor(
                String::from("abcdef"),
                0,
                3,
                )
            );
        editor.delete_at(10, 10);
        assert_eq!(
            editor,
            build_editor(
                String::from("abcdef"),
                0,
                3,
                )
            );
    }
}
