use std::io::Read;

use crate::editor::Editor;

mod editor;
mod terminal;
mod view;
mod buffer;
mod editorcommand;
mod location;
mod line;
mod uicomponent;
mod statusbar;
mod fileinfo;
mod documentstatus;

fn main() {
    Editor::new().unwrap().run();
}