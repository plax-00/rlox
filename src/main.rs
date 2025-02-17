#![allow(dead_code)]
use std::env;

#[cfg(test)]
mod ast_print;
mod error;
mod expression;
mod interpreter;
mod operator;
mod parser;
mod scanner;
mod token;
mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len() - 1);
}
