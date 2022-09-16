use crossterm::event::{KeyCode, KeyEvent};
use indicatif::ProgressBar;

pub mod ui;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub total_time: Vec<u16>,
    pub total_time_m: u16,
    pub progress: u16,
    pub time_bar: ProgressBar,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            total_time: Vec::new(),
            total_time_m: 0,
            progress: 0,
            time_bar: ProgressBar::new(10)
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
                self.total_time.push(
                    self.input
                        .trim()
                        .parse::<u16>()
                        .unwrap()
                );

                self.set_normal_mode()
            },
            KeyCode::Backspace => {
                self.input.pop();

                return
            },

            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.progress += 100 / self.total_time[0];

        if self.progress > 100 {
            self.progress = 100;
        }
    }
}
