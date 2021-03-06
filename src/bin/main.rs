//! The experimental playground of two-dimensional editing facility.
#![warn(missing_docs)]

extern crate edit;
use edit::editor::*;

fn main() {
    let mut editor = Editor::new("Hello, world!\nThe 2nd line.", 1, 6).unwrap();
    editor.move_to_end();
    editor.move_right(1);
    editor.move_left(2);
    editor.move_to_beginning();
    editor.move_up(1);
    editor.move_down(4);
    editor.insert_at('4', 1, 4);
    editor.insert_at('4', 0, 0);
    editor.delete_at(0, 13);
    editor.delete_range(
        Position {
            line: 0,
            column: 13,
        }..Position { line: 1, column: 2 },
    );
    let width = editor.line_width(0).unwrap();
    editor.insert_string_at("\nThe 3rd line.", 1, width);
    let buffer: String = editor.buffer().iter().collect();
    println!("editor: {} {} {}", buffer, editor.line(), editor.column())
}
