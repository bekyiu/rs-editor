use std::io::Error;
use crate::documentstatus::DocumentStatus;
use crate::terminal::{Size, Terminal};
use crate::uicomponent::UIComponent;

#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for StatusBar {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    // 画在origin_y这一行
    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        // Assemble the first part of the status bar
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let beginning = format!(
            "{} - {line_count} {modified_indicator}",
            self.current_status.filename
        );
        // Assemble the whole status bar, with the position indicator at the back
        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>remainder_len$}");
        // Only print out the status if it fits. Otherwise write out an empty string to ensure the row is cleared.
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };
        let result = Terminal::print_inverted_row(origin_y, &to_print);
        debug_assert!(result.is_ok(), "Failed to render status bar");
        result
    }
}