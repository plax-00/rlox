use std::{error::Error, fmt::Display};

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
