use std::env;

mod run;
use run::{repl, run_file};

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
