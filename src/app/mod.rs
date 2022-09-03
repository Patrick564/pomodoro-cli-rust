use crossterm::event::{KeyCode, KeyEvent};

pub mod ui;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

impl App {
    pub fn set_editing_mode(&mut self) {
        self.input_mode = InputMode::Editing
    }

    pub fn set_normal_mode(&mut self) {
        self.input_mode = InputMode::Normal
    }

    pub fn handle_editing_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.input.push(c),

            KeyCode::Esc => self.set_normal_mode(),
            KeyCode::Enter => {
                self.messages.push(self.input.drain(..).collect());

                self.set_normal_mode()
            },
            KeyCode::Backspace => {
                self.input.pop();

                return
            },

            _ => {}
        }
    }
}
