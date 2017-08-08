mod editor;
use editor::*;

fn main() {
    let mut editor = new(String::from("Hello, world!\nThe 2nd line."), 1, 6).unwrap();
    editor.move_right(1);
    editor.move_left(2);
    editor.move_up(1);
    editor.move_down(4);
    editor.insert_at('4', 1, 4);
    editor.insert_at('4', 0, 0);
    println!(
        "editor: {} {} {}",
        editor.buffer(),
        editor.line(),
        editor.column()
    )
}
