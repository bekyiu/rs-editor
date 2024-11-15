use std::fs;
use std::io::Error;
use crate::line::Line;
use crate::location::Location;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
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
        Ok(Self { lines })
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
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            // 一行的中间插入
            line.insert_char(character, at.grapheme_index);
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            line.delete(at.grapheme_index);

        }
    }
}
