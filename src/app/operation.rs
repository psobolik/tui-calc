/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-07-08
 */
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Operation {
    #[default]
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
            Operation::Equal => "=",
        };
        write!(f, "{}", symbol)
    }
}
