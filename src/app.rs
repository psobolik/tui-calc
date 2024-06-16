pub use self::{digits::Digits, digits::PushError};

use self::operation::Operation;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::error;
use std::fmt::Debug;

/// Event handler.
pub mod digits;
mod operation;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    digits: Digits,
    acc: f64,
    current_operation: Operation,
    history: Vec<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn push_digit(&mut self, digit: char) {
        if "0123456789".contains(digit) {
            // Silently ignore problems (too many digits, etc.)
            if self.digits.push(digit).is_ok() && self.current_operation == Operation::Equal {
                self.acc = 0_f64;
            }
        }
    }

    pub fn decimal(&mut self) {
        self.digits.set_decimal_position();
    }

    pub fn backspace(&mut self) {
        let _ = self.digits.pop();
    }

    pub fn clear(&mut self) {
        if self.current_operation != Operation::default() {
            self.current_operation = Operation::default();
        } else if !self.digits.is_empty() {
            self.digits.clear();
        } else {
            self.acc = 0_f64;
        }
    }

    pub fn acc(&self) -> f64 {
        self.acc
    }

    pub fn digits(&self) -> String {
        self.digits.to_string()
    }

    pub fn number(&self) -> f64 {
        (&self.digits).into()
    }

    pub fn make_negative(&mut self) {
        self.digits.set_negative(true);
    }

    pub fn make_positive(&mut self) {
        self.digits.set_negative(false);
    }

    pub fn add(&mut self) {
        self.perform_operation();
        self.current_operation = Operation::Add;
    }

    pub fn subtract(&mut self) {
        self.perform_operation();
        self.current_operation = Operation::Subtract;
    }

    pub fn multiply(&mut self) {
        self.perform_operation();
        self.current_operation = Operation::Multiply;
    }

    pub fn divide(&mut self) {
        self.perform_operation();
        self.current_operation = Operation::Divide;
    }

    pub fn equal(&mut self) {
        self.perform_operation();
        self.current_operation = Operation::Equal;
    }

    fn perform_operation(&mut self) {
        if self.current_operation == Operation::Equal && self.digits.is_empty() {
            return;
        }

        let acc = self.acc;
        match self.current_operation {
            Operation::Add => self.acc += self.number(),
            Operation::Subtract => self.acc -= self.number(),
            Operation::Multiply => self.acc *= self.number(),
            Operation::Divide => self.acc /= self.number(),
            Operation::Equal => self.acc = self.number(),
        }
        if self.current_operation != Operation::Equal {
            self.history.push(format!(
                "{} {} {} = {}",
                acc,
                self.current_operation(),
                self.number(),
                self.acc
            ));
        }
        self.digits.clear();
        self.current_operation = Operation::default();
    }

    pub fn history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn current_operation(&self) -> String {
        match self.current_operation {
            Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide => {
                self.current_operation.to_string()
            }
            Operation::Equal => String::default(),
        }
    }
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        match key_event.code {
            // Exit application on Ctrl+C or Ctrl+Q
            KeyCode::Char('q') | KeyCode::Char('c')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                self.quit()
            }
            KeyCode::Char('c') | KeyCode::Esc => self.clear(),
            KeyCode::Char('+') => self.add(),
            KeyCode::Char('-') => self.subtract(),
            KeyCode::Char('*') => self.multiply(),
            KeyCode::Char('/') => self.divide(),
            KeyCode::Char('=') | KeyCode::Enter => self.equal(),
            KeyCode::Char('n') | KeyCode::Char('N') => self.make_negative(),
            KeyCode::Char('p') | KeyCode::Char('P') => self.make_positive(),
            KeyCode::Char('.') => self.decimal(),
            KeyCode::Char(c) => self.push_digit(c),
            KeyCode::Backspace => self.backspace(),
            _ => {}
        }
        Ok(())
    }
}
