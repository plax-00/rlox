use std::{error::Error, fmt::Display};

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    pub line_num: u32,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "syntax error on line {}", self.line_num)
    }
}

impl Error for SyntaxError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        "Source code contains a syntax error."
    }
}

#[derive(Debug)]
pub struct OperatorParseError<'a> {
    pub token: &'a Token,
}

impl Display for OperatorParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing operator from token: {:?}", self.token)
    }
}

impl Error for OperatorParseError<'_> {
    fn description(&self) -> &str {
        "Token is an invalid operator."
    }
}
