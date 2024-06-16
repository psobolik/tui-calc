/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-06-15
 */

use std::fmt::{Display, Formatter};

use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget, text::Line};

use super::PushError::{InvalidDigit, TooManyDigits};

/// Models a number as a vector of digits, possibly with a decimal position and with a flag
/// to indicate the number is negative.
#[derive(Clone, Debug, Default)]
pub struct Digits {
    is_negative: bool,
    digits: Vec<u8>,
    decimal_position: Option<usize>,
}

pub enum PushError {
    InvalidDigit(char),
    TooManyDigits,
}

impl Digits {
    const MAX_DIGITS: usize = 15;

    pub fn push(&mut self, digit: char) -> Result<(), PushError> {
        if self.digits.len() >= Self::MAX_DIGITS {
            Err(TooManyDigits)
        } else {
            match digit {
                // Prevent leading zeros before the decimal.
                '0' if !self.digits.is_empty() || self.decimal_position.is_some() => {
                    self.digits.push(b'0');
                    Ok(())
                }
                '1'..='9' => {
                    self.digits.push(digit as u8);
                    Ok(())
                }
                _ => Err(InvalidDigit(digit)),
            }
        }
    }

    pub fn pop(&mut self) -> Option<char> {
        if Some(self.digits.len()) == self.decimal_position {
            self.decimal_position = None;
            Some('.')
        } else {
            match self.digits.pop() {
                None => None,
                Some(byte) => {
                    if self.digits.is_empty() {
                        self.set_negative(false)
                    }
                    char::from_digit(byte as u32, 10)
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.set_negative(false);
        self.decimal_position = None;
        self.digits.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    pub fn set_negative(&mut self, flag: bool) {
        self.is_negative = flag
    }

    pub fn toggle_negative(&mut self) {
        self.set_negative(self.is_negative())
    }

    pub fn set_decimal_position(&mut self) {
        if self.decimal_position.is_none() {
            self.decimal_position = Some(self.digits.len());
        }
    }
}

impl From<f64> for Digits {
    fn from(value: f64) -> Self {
        let mut result = Self::default();
        let value = value.to_string();
        value.as_bytes().iter().for_each(|byte| {
            match byte {
                b'-' => result.set_negative(true),
                b'.' => result.set_decimal_position(),
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    result.push(*byte as char).unwrap_or_default()
                }
                _ => panic!("Invalid digit"), // This will never happen
            }
        });
        result
    }
}

impl From<&Digits> for f64 {
    fn from(value: &Digits) -> Self {
        const BASE: u128 = 10;
        let len = value.digits.len();

        let i = value
            .digits
            .iter()
            .fold(0u128, |acc, &d| acc * BASE + ((d - b'0') as u128)) as f64;
        let f = match value.decimal_position {
            None => 1_f64,
            Some(decimal) => BASE.pow((len - decimal) as u32) as f64,
        };
        (i / f) * if value.is_negative() { -1_f64 } else { 1_f64 }
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_negative() {
            f.write_str("-")?;
        };
        match self.decimal_position {
            Some(decimal) => {
                let left = if decimal > 0 {
                    std::str::from_utf8(&self.digits[..decimal]).unwrap_or_default()
                } else {
                    "0"
                };
                let right = std::str::from_utf8(&self.digits[decimal..]).unwrap_or_default();
                write!(f, "{}.{}", left, right)
                // f.write_fmt(format_args!("{}.{}", left, right))
            }
            _ => f.write_str(match self.digits.len() {
                0 => "0",
                _ => std::str::from_utf8(&self.digits).unwrap_or_default(),
            }),
        }
    }
}
impl Widget for Digits {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let value = self.to_string();
        let x = area.width.saturating_sub(value.len() as u16);
        let y = area.y + (area.height.saturating_sub(1) / 2);
        buf.set_line(x, y, &Line::from(value.to_string()), value.len() as u16);
    }
}
