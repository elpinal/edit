struct Editor {
    buffer: String,
    position: u32,
}

fn move_left(editor: Editor, n: u32) -> Editor {
    Editor {
        position: editor.position-n,
        ..editor
    }
}

fn move_right(editor: Editor, n: u32) -> Editor {
    Editor {
        position: editor.position+n,
        ..editor
    }
}

fn main() {
    let editor = Editor {
        buffer: String::from("Hello, world!"),
        position: 6,
    };
    let editor = move_right(editor, 1);
    let editor = move_left(editor, 2);
    println!("editor: {} {}", editor.buffer, editor.position);
}
