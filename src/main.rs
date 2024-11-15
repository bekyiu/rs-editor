use std::io::Read;

use crate::editor::Editor;

mod editor;
mod terminal;
mod view;
mod buffer;
mod editorcommand;
mod location;
mod line;

fn main() {
    Editor::new().unwrap().run();
}