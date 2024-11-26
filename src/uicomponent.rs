use std::io::Error;
use crate::terminal::Size;

pub trait UIComponent {
    // 标记是否是要重绘
    fn mark_redraw(&mut self, value: bool);
    // 判断是否可以重绘
    fn needs_redraw(&self) -> bool;

    fn set_size(&mut self, size: Size);

    fn draw(&mut self, origin_y: usize) -> Result<(), Error>;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.mark_redraw(true);
    }

    fn render(&mut self, origin_y: usize) {
        if self.needs_redraw() {
            match self.draw(origin_y) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }
}