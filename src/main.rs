struct Editor {
    buffer: String,
    line: u32,
    column: u32,
}

fn build_editor(buffer: String, line: u32, column: u32) -> Editor {
    Editor {
        buffer,
        line,
        column,
    }
}

fn move_left(editor: Editor, n: u32) -> Editor {
    Editor {
        column: editor.column-n,
        ..editor
    }
}

fn move_right(editor: Editor, n: u32) -> Editor {
    Editor {
        column: editor.column+n,
        ..editor
    }
}

fn main() {
    let editor = build_editor(
        String::from("Hello, world!"),
        0, 
        6,
    );
    let editor = move_right(editor, 1);
    let editor = move_left(editor, 2);
    println!("editor: {} {}", editor.buffer, editor.column);
}
