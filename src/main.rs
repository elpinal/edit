mod editor {
#[derive(PartialEq, Debug)]
    pub struct Editor {
        buffer: String,
        newline_indices: Vec<u32>,
        line: u32,
        column: u32,
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

    pub fn move_left(editor: Editor, n: u32) -> Editor {
        if editor.column < n {
            return Editor {
                column: 0,
                ..editor
            };
        }
        Editor {
            column: editor.column-n,
            ..editor
        }
    }

    pub fn move_right(editor: Editor, n: u32) -> Editor {
        // TODO: How about try_from?
        let line_len = editor.newline_indices[editor.line as usize] - if editor.line == 0 {
            0
        } else {
            editor.newline_indices[(editor.line - 1) as usize]
        };
        if editor.column + n >= line_len {
            return Editor {
                column: line_len - 1,
                ..editor
            };
        }
        Editor {
            column: editor.column+n,
            ..editor
        }
    }

    pub fn move_up(editor: Editor, n: u32) -> Editor {
        if editor.line < n {
            return Editor {
                line: 0,
                ..editor
            };
        }
        Editor {
            line: editor.line-n,
            ..editor
        }
    }

    pub fn move_down(editor: Editor, n: u32) -> Editor {
        let len: u32 = editor.newline_indices.len() as u32;
        if editor.line + n >= len {
            return Editor {
                line: len - 1,
                ..editor
            };
        }
        Editor {
            line: editor.line+n,
            ..editor
        }
    }

    pub fn insert_at(editor: Editor, ch: char, line: u32, column: u32) -> Editor {
        let mut buffer: String = editor.buffer;
        let line_offset = if line == 0 {
            0
        } else {
            editor.newline_indices[(line-1) as usize] + 1
        };
        let i: usize = (line_offset + column) as usize;
        buffer.insert(i, ch);
        if ch == '\n' {
            let mut indices = &mut editor.newline_indices.clone();
            indices.insert(line as usize, i as u32);
            let v: Vec<u32> = indices.into_iter().map(|x| if *x > i as u32 {*x + 1} else {*x}).collect();
            return Editor {
                buffer,
                newline_indices: v,
                ..editor
            };
        }
        Editor {
            buffer: buffer,
            ..editor
        }
    }

#[cfg(test)]
    mod tests {
        use super::*;

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
                editor = move_right(editor, 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        1,
                        expected[i],
                        ));
            }

            for i in 0..(buffer.len() - buffer.rfind('\n').unwrap()){
                let editor = build_editor(
                    String::from(buffer),
                    1,
                    i as u32,
                    );
                let editor = move_right(editor, buffer.len() as u32 + 1);
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
                editor = move_left(editor, 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        1,
                        expected[i],
                        ));
            }

            for i in 0..(buffer.len() - buffer.rfind('\n').unwrap()){
                let editor = build_editor(
                    String::from(buffer),
                    1,
                    i as u32,
                    );
                let editor = move_left(editor, buffer.len() as u32 + 1);
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
                editor = move_up(editor, 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        expected[i],
                        4,
                        ));
            }

            for i in 0..(buffer.match_indices('\n').count() + 1) {
                let editor = build_editor(
                    String::from(buffer),
                    i as u32,
                    1,
                    );
                let editor = move_up(editor, buffer.len() as u32 + 1);
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
                editor = move_down(editor, 1);
                assert_eq!(editor, build_editor(
                        String::from(buffer),
                        expected[i],
                        4,
                        ));
            }

            for i in 0..(buffer.match_indices('\n').count() + 1) {
                let editor = build_editor(
                    String::from(buffer),
                    i as u32,
                    1,
                    );
                let editor = move_down(editor, buffer.len() as u32 + 1);
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
            editor = insert_at(editor, '\n', 0, 6);
            assert_eq!(editor, build_editor(
                    String::from("Hello,\n world!\nThe 2nd line.\nAAABBBCCC."),
                    0,
                    6,
                    ));
        }
    }
}

use editor::*;

fn main() {
    let editor = build_editor(
        String::from("Hello, world!\nThe 2nd line."),
        1,
        6,
        );
    let editor = move_right(editor, 1);
    let editor = move_left(editor, 2);
    let editor = move_up(editor, 1);
    let editor = move_down(editor, 4);
    let editor = insert_at(editor, '4', 1, 4);
    let editor = insert_at(editor, '4', 0, 0);
    println!("editor: {:?}", editor)
}
