use std::char;

use crossterm::event;

pub enum Key {
    Ctrl(char),
    Char(char),
    Unknow
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            // Char + modifier check
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),

            // Char check
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),

            // Default
            _ => Key::Unknow,
        }
    }
}
