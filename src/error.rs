use std::{error::Error, fmt::Display};

use crate::token::TokenType;

impl Error for SyntaxError {}
impl Error for ParseError {}
impl Error for RuntimeError {}

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    pub line_num: u32,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "syntax error on line {}", self.line_num)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub error_type: ParseErrorType,
    pub line_num: u32,
}

#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ParseErrorType::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "line {}: Expected `{}`, found `{}`",
                    self.line_num,
                    expected.lexeme(),
                    found.lexeme()
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub line_num: u32,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: {}", self.line_num)
    }
}
