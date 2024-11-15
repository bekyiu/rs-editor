use std::cmp::min;

use crate::buffer::Buffer;
use crate::editorcommand::{Direction, EditorCommand};
use crate::line::Line;
use crate::location::Location;
use crate::terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct View {
    // 保存绘制的文本
    buffer: Buffer,
    // 是否需要重绘
    needs_redraw: bool,
    // 窗口大小
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;
        let top = self.scroll_offset.row;

        for current_row in 0..height {
            if let Some(text) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &text.get_visible_graphemes(left..right));
            } else {
                if current_row == vertical_center && self.buffer.is_empty() {
                    Self::render_line(current_row, &Self::build_welcome_message(width));
                } else {
                    Self::render_line(current_row, "~");
                }
            }
        }

        self.needs_redraw = false;
    }


    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.needs_redraw = true;
    }

    pub fn handle_command(&mut self, cmd: EditorCommand) {
        match cmd {
            EditorCommand::Move(dir) => self.move_text_location(dir),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Insert(ch) => self.insert_char(ch),
            EditorCommand::Backspace => self.backspace(),
            EditorCommand::Delete => self.delete(),
            EditorCommand::Quit => {}
        }
    }

    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn backspace(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.move_text_location(Direction::Left);
            self.delete();
        }
    }

    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.needs_redraw = true;
    }

    fn insert_char(&mut self, ch: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        self.buffer.insert_char(ch, self.text_location);
        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            //move right for an added grapheme (should be the regular case)
            self.move_text_location(Direction::Right);
        }
        self.needs_redraw = true;
    }

    fn move_text_location(&mut self, dir: Direction) {
        let Size { height, .. } = self.size;

        // This match moves the position, but does not check for all boundaries.
        // The final boundarline checking happens after the match statement.
        match dir {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_of_line(),
            Direction::End => self.move_to_end_of_line(),
        }

        self.scroll_location_into_view();
    }

    // 上移, 注意光标位置
    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        // 光标所在行的文本
        let line_width = self.buffer.lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    // 确保grapheme_index指向正确的grapheme
    // 不触发滚动
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index =
            self.buffer.lines.get(self.text_location.line_index)
                .map_or(0, |line| {
                    // 从长的行移到短的行, 要保证光标不能超出较短行的末尾
                    min(line.grapheme_count(), self.text_location.grapheme_index)
                });
    }

    // 确保line_index指向正确的line
    // 不触发滚动
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height())
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, width } = self.size;
        let mut offset_changed = false;
        if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            offset_changed = true;
        } else if to >= self.scroll_offset.row + height {
            self.scroll_offset.row = to - height + 1;
            offset_changed = true;
        }

        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { height, width } = self.size;
        let mut offset_changed = false;
        if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            offset_changed = true;
        } else if to >= self.scroll_offset.col + width {
            self.scroll_offset.col = to - width + 1;
            offset_changed = true;
        }
        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    // 修正offset
    fn scroll_location_into_view(&mut self) {
        // 一个grapheme在屏幕上占据多个位置(列), 所以要转换为position再计算offset
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    fn render_line(row: usize, text: &str) {
        let result = Terminal::print_row(row, text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();

        // 如果不够显示就不显示了
        if width <= len {
            return "~".to_string();
        }
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        welcome_message
    }
}