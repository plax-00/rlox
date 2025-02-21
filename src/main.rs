#![allow(dead_code)]
use std::io::{self, Write};

use anyhow::Result;
use colored::Colorize;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

#[cfg(test)]
mod ast_print;
mod environment;
mod error;
mod expression;
mod interpreter;
mod operator;
mod parser;
mod scanner;
mod statement;
mod token;
mod value;

fn main() -> Result<()> {
    // clear screen
    print!("\x1B[2J\x1B[1;1H");
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut int = Interpreter::default();

    // repl
    let mut buf = String::new();
    loop {
        print!("{}", "rlox".bold().green());
        print!("{}", " > ".purple());
        stdout.flush()?;

        buf.clear();
        stdin.read_line(&mut buf)?;
        if buf == "\n" {
            continue;
        }

        let tokens = match Scanner::new(buf.clone()).scan_source() {
            Ok(t) => t,
            Err(errs) => {
                for e in errs {
                    eprintln!("{}", e.to_string().red());
                }
                continue;
            }
        };
        let stmts = match Parser::new(tokens).parse() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                continue;
            }
        };
        if let Err(e) = int.interpret(&stmts) {
            eprintln!("{}", e.to_string().red());
        }
    }
}

macro_rules! impl_from_inner {
    ($from:ident, $for:ty) => {
        impl From<$from> for $for {
            fn from(value: $from) -> Self {
                Self::$from(value)
            }
        }
    };
}

pub(crate) use impl_from_inner;
