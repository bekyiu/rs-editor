use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Insert(char),
    Backspace,
    Delete,
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(
                KeyEvent {
                    code, modifiers, kind: KeyEventKind::Press, ..
                }) => {
                match code {
                    KeyCode::Char('q')
                    if modifiers == KeyModifiers::CONTROL => {
                        Ok(EditorCommand::Quit)
                    }
                    KeyCode::Char(ch)
                    if modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT => {
                        Ok(EditorCommand::Insert(ch))
                    }
                    KeyCode::Up => Ok(EditorCommand::Move(Direction::Up)),
                    KeyCode::Down => Ok(EditorCommand::Move(Direction::Down)),
                    KeyCode::Left => Ok(EditorCommand::Move(Direction::Left)),
                    KeyCode::Right => Ok(EditorCommand::Move(Direction::Right)),
                    KeyCode::PageDown => Ok(EditorCommand::Move(Direction::PageDown)),
                    KeyCode::PageUp => Ok(EditorCommand::Move(Direction::PageUp)),
                    KeyCode::End => Ok(EditorCommand::Move(Direction::End)),
                    KeyCode::Home => Ok(EditorCommand::Move(Direction::Home)),
                    KeyCode::Delete => Ok(EditorCommand::Delete),
                    KeyCode::Backspace => Ok(EditorCommand::Backspace),
                    _ => Err(format!("Key Code not supported: {code:?}")),
                }
            }
            Event::Resize(col, row) => {
                let size = Size {
                    width: col as usize,
                    height: row as usize,
                };
                Ok(EditorCommand::Resize(size))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
