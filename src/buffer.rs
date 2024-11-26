use std::fs;
use std::fs::File;
use std::io::Error;
use crate::line::Line;
use crate::location::Location;
use std::io::Write;
use crate::fileinfo::FileInfo;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_info: FileInfo,
    pub dirty: bool,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self {
            lines,
            file_info: FileInfo::from(filename),
            dirty: false,
        })
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.lines.len() {
            return;
        }
        // 在最后一行插入
        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(character.to_string().as_str()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            // 一行的中间插入
            line.insert_char(character, at.grapheme_index);
            self.dirty = true;
        }
    }

    pub fn delete(&mut self, at: Location) {
        // 这里没法get_mut
        if let Some(line) = self.lines.get(at.line_index) {
            // 光标在行首 按一下 backspace之后的状态:
            // 光标在末尾 并且 不是在最后一行
            if at.grapheme_index >= line.grapheme_count()
                && self.lines.len() > at.line_index.saturating_add(1)
            {
                // 把下一行合并到当前行
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                self.lines[at.line_index].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_index < line.grapheme_count() {
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }

    pub fn insert_newline(&mut self, at: Location) {
        if at.line_index == self.height() {
            // 末尾插入一行
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            // 把剩下的部分插到下一行
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
            self.dirty = true;
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.file_info.path {
            let mut file = File::create(filename)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
}
