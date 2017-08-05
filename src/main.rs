mod editor {
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
            Some(self.newline_indices[n as usize] - if n == 0 {
                0
            } else {
                self.newline_indices[n as usize - 1] + 1
            })
        }
        pub fn offset(&self, line: u32, column: u32) -> Option<u32> {
            if line >= self.line_count() || self.line_width(line).unwrap() < column {
                return None;
            };
            let line_offset = if line == 0 {
                0
            } else {
                self.newline_indices[(line-1) as usize] + 1
            };
            Some(line_offset + column)
        }

        pub fn move_left(&mut self, n: u32) {
            if self.column < n {
                self.column = 0;
                return
            }
            self.column -= n;
        }

        pub fn move_right(&mut self, n: u32) {
            if self.column + n >= self.line_width(self.line).unwrap() {
                self.column = self.line_width(self.line).unwrap();
                return
            }
            self.column += n;
        }

        pub fn move_up(&mut self, n: u32) {
            if self.line < n {
                self.line = 0;
                return
            }
            self.line-=n;
        }

        pub fn move_down(&mut self, n: u32) {
            if self.line + n >= self.line_count() {
                    self.line = self.line_count() - 1;
                    return
            }
            self.line += n;
        }

        pub fn insert_at(&mut self, ch: char, line: u32, column: u32) {
            let i = self.offset(line, column).unwrap();
            self.buffer.insert(i as usize, ch);
            if ch != '\n' {
                return
            }
            self.newline_indices.insert(line as usize, i);
            for x in self.newline_indices.iter_mut() {
                if *x > i {
                    *x += 1
                }
            }
        }

        pub fn delete_at(&mut self, line: u32, column: u32) {
            let offset = self.offset(line, column).unwrap();
            self.buffer.remove(offset as usize);
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
            let editor = build_editor(
                String::from(buffer),
                0,
                0,
                );
            assert_eq!(editor.line_count(), 2);
        }

        #[test]
        fn test_line_width() {
            let buffer = "Hello, world!\nThe 2nd line.";
            let editor = build_editor(
                String::from(buffer),
                0,
                0,
                );
            assert_eq!(editor.line_width(0), Some(13));
            assert_eq!(editor.line_width(1), Some(13));
            assert_eq!(editor.line_width(2), None);
        }

        #[test]
        fn test_offset() {
            let buffer = "Hello, world!\nThe 2nd line.";
            let editor = build_editor(
                String::from(buffer),
                0,
                0,
                );
            assert_eq!(editor.offset(0, 0), Some(0));
            assert_eq!(editor.offset(1, 1), Some(15));
            assert_eq!(editor.offset(2, 2), None);
            assert_eq!(editor.offset(1, 13), Some(27));
            assert_eq!(editor.offset(1, 14), None);
            assert_eq!(editor.offset(0, 13), Some(13));
            assert_eq!(editor.offset(0, 14), None);
        }

        #[test]
        fn test_move_right() {
            let buffer = "Hello, world!\nThe 2nd line.";
            let mut editor = build_editor(
                String::from(buffer),
                1,
                6,
                );
            let expected = [7, 8, 9, 10, 11, 12, 13, 13];
            for i in 0..expected.len() {
                editor.move_right(1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        1,
                        expected[i],
                        ));
            }

            for i in 0..(buffer.len() - buffer.rfind('\n').unwrap()){
                let mut editor = build_editor(
                    String::from(buffer),
                    1,
                    i as u32,
                    );
                editor.move_right(buffer.len() as u32 + 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        1,
                        buffer.rfind('\n').unwrap() as u32,
                        ));
            }
        }

        #[test]
        fn test_move_left() {
            let buffer = "Hello, world!\nThe 2nd line.";
            let mut editor = build_editor(
                String::from(buffer),
                1,
                6,
                );
            let expected = [5, 4, 3, 2, 1, 0, 0];
            for i in 0..expected.len() {
                editor.move_left(1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        1,
                        expected[i],
                        ));
            }

            for i in 0..(buffer.len() - buffer.rfind('\n').unwrap()){
                let mut editor = build_editor(
                    String::from(buffer),
                    1,
                    i as u32,
                    );
                editor.move_left(buffer.len() as u32 + 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        1,
                        0,
                        ));
            }
        }

        #[test]
        fn test_move_up() {
            let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
            let mut editor = build_editor(
                String::from(buffer),
                2,
                4,
                );
            let expected = [1, 0, 0];
            for i in 0..expected.len() {
                editor.move_up(1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        expected[i],
                        4,
                        ));
            }

            for i in 0..(buffer.match_indices('\n').count() + 1) {
                let mut editor = build_editor(
                    String::from(buffer),
                    i as u32,
                    1,
                    );
                editor.move_up(buffer.len() as u32 + 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        0,
                        1,
                        ));
            }
        }

        #[test]
        fn test_move_down() {
            let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
            let mut editor = build_editor(
                String::from(buffer),
                0,
                4,
                );
            let expected = [1, 2, 2];
            for i in 0..expected.len() {
                editor.move_down(1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        expected[i],
                        4,
                        ));
            }

            for i in 0..(buffer.match_indices('\n').count() + 1) {
                let mut editor = build_editor(
                    String::from(buffer),
                    i as u32,
                    1,
                    );
                editor.move_down(buffer.len() as u32 + 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        buffer.match_indices('\n').count() as u32,
                        1,
                        ));
            }
        }

        #[test]
        fn test_insert_at() {
            let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
            let mut editor = build_editor(
                String::from(buffer),
                0,
                6,
                );
            editor.insert_at('\n', 0, 6);
            assert_eq!(editor, build_editor(
                    String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCC."),
                    0,
                    6,
                    ));
        }

        #[test]
        fn test_delete_at() {
            let buffer = "Hello, world!\nThe 2nd line.\nAAABBBCCC.";
            let mut editor = build_editor(
                String::from(buffer),
                0,
                6,
                );
            editor.delete_at(0, 6);
            assert_eq!(editor, build_editor(
                    String::from("Hello,world!\nThe 2nd line.\nAAABBBCCC."),
                    0,
                    6,
                    ));
        }
    }
}

use editor::*;

fn main() {
    let mut editor = build_editor(
        String::from("Hello, world!\nThe 2nd line."),
        1,
        6,
        );
    editor.move_right(1);
    editor.move_left(2);
    editor.move_up(1);
    editor.move_down(4);
    editor.insert_at('4', 1, 4);
    editor.insert_at('4', 0, 0);
    println!("editor: {} {} {}", editor.buffer(), editor.line(), editor.column())
}
