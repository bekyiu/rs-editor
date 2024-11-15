use std::env;
use std::io::{Error, Read};
use std::panic::{set_hook, take_hook};

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};

use crate::editorcommand::EditorCommand;
use crate::terminal::Terminal;
use crate::view::View;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        // 原本panic时的回调
        let current_hook = take_hook();
        // 设置新的回调
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut this = Self {
            should_quit: false,
            view: View::default(),
        };
        this.handle_args();
        Ok(this)
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    // debug模式下才会编译执行
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename);
        }
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(cmd) = EditorCommand::try_from(event) {
                if matches!(cmd, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(cmd);
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                // panic!("Received and discarded unsupported or non-press event.");
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
