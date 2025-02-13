#![allow(dead_code)]
use std::env;

mod ast_print;
mod error;
mod expression;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len() - 1);
}
