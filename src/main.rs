#![feature(test)]

mod editor;
use editor::*;

fn main() {
    let mut editor = new(String::from("Hello, world!\nThe 2nd line."), 1, 6).unwrap();
    editor.move_to_end();
    editor.move_right(1);
    editor.move_left(2);
    editor.move_to_beginning();
    editor.move_up(1);
    editor.move_down(4);
    editor.insert_at('4', 1, 4);
    editor.insert_at('4', 0, 0);
    editor.delete_at(0, 13);
    let width = editor.line_width(1).unwrap();
    editor.insert_string_at("\nThe 3rd line.", 1, width);
    println!(
        "editor: {} {} {}",
        editor.buffer(),
        editor.line(),
        editor.column()
    )
}
