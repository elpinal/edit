mod core;

use editor::core::Core;

pub struct Editor {
    core: Core,
}

pub fn new(buffer: String, line: u32, column: u32) -> Result<Editor, String> {
    match core::new(buffer, line, column) {
        Ok(editor) => Ok(Editor { core: editor }),
        Err(err) => Err(err),
    }
}

impl Editor {
    pub fn move_right(&mut self, n: u32) {
        self.core.move_right(n);
    }

    pub fn move_left(&mut self, n: u32) {
        self.core.move_left(n);
    }

    pub fn move_up(&mut self, n: u32) {
        self.core.move_up(n);
    }

    pub fn move_down(&mut self, n: u32) {
        self.core.move_down(n);
    }

    pub fn insert_at(&mut self, ch: char, line: u32, column: u32) {
        self.core.insert_at(ch, line, column);
    }

    pub fn buffer(&self) -> String {
        self.core.buffer()
    }

    pub fn line(&self) -> u32 {
        self.core.line()
    }

    pub fn column(&self) -> u32 {
        self.core.column()
    }
}
