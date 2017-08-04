#[derive(PartialEq, Debug)]
struct Editor {
    buffer: String,
    newline_indices: Vec<u32>,
    line: u32,
    column: u32,
}

fn build_editor(buffer: String, line: u32, column: u32) -> Editor {
    let mut indices: Vec<u32> = buffer.match_indices('\n').map(|(a, _)| a as u32).collect();
    indices.push(buffer.len() as u32);
    Editor {
        buffer: buffer.clone(),
        newline_indices: indices,
        line,
        column,
    }
}

fn move_left(editor: Editor, n: u32) -> Editor {
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

fn move_right(editor: Editor, n: u32) -> Editor {
    // TODO: How about try_from?
    let line_offset = if editor.line == 0 {
        0
    } else {
        editor.newline_indices[(editor.line - 1) as usize]
    };
    if editor.column + n > line_offset {
        return Editor {
            column: line_offset,
            ..editor
        };
    }
    Editor {
        column: editor.column+n,
        ..editor
    }
}

fn move_up(editor: Editor, n: u32) -> Editor {
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

fn move_down(editor: Editor, n: u32) -> Editor {
    let len: u32 = editor.newline_indices.len() as u32;
    if editor.line + n > len {
        return Editor {
            line: len - 1,
            ..editor
        };
    }
    Editor {
        line: editor.line-n,
        ..editor
    }
}

fn insert_at(editor: Editor, ch: char, line: u32, column: u32) -> Editor {
    let mut buffer: String = editor.buffer;
    let line_offset = if line == 0 {
        0
    } else {
        editor.newline_indices[(line-1) as usize] + 1
    };
    let i: usize = (line_offset + column) as usize;
    buffer.insert(i, ch);
    Editor {
        buffer: buffer,
        ..editor
    }
}

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
    println!("editor: {} {} {}", editor.buffer, editor.line, editor.column);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let mut editor = build_editor(
            String::from("Hello, world!\nThe 2nd line."),
            1,
            6,
        );
        let expected = [7, 8, 9, 10, 11, 12, 13, 13];
        for i in 0..expected.len() {
            editor = move_right(editor, 1);
            assert_eq!(editor, build_editor(
                    String::from("Hello, world!\nThe 2nd line."),
                    1,
                    expected[i],
            ));
        }
    }

    #[test]
    fn test_move_left() {
        let mut editor = build_editor(
            String::from("Hello, world!\nThe 2nd line."),
            1,
            6,
        );
        let expected = [5, 4, 3, 2, 1, 0, 0];
        for i in 0..expected.len() {
            editor = move_left(editor, 1);
            assert_eq!(editor, build_editor(
                    String::from("Hello, world!\nThe 2nd line."),
                    1,
                    expected[i],
            ));
        }
    }
}
