#![allow(dead_code)]

#[macro_use]
mod macros;

mod chunk;
mod compiler;
mod debug;
mod lexer;
mod run;
mod vm;

use run::{repl, run_file};
use std::env;

pub const DEBUG_LEXER: bool = false;
pub const DEBUG_TRACE: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_args = args.len();

    match n_args {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: ./rlox [script.lox]");
        }
    }
}
