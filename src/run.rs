// use crate::vm::VirtualMachine;

#[allow(dead_code)]
pub fn repl() {
	use rustyline::error::ReadlineError;
	use rustyline::Editor;

	// let vm = VirtualMachine::new();
	let mut rl = Editor::<()>::new();
	loop {
		let readline = rl.readline(">> ");
		match readline {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				println!("{}", line);
			},
			Err(ReadlineError::Interrupted) => break,
			Err(ReadlineError::Eof)         => break,
			Err(err) => {
				println!("Error: {:?}", err);
				break
			}
		}
	}
}

#[allow(dead_code)]
pub fn run_file(filename: &str) {
	use std::io::Read;
	use std::fs::File;

	println!("running file: {}", filename);

	let mut buffer = String::new();
	{
		File::open(&filename)
			.expect("failed to open file")
			.read_to_string(&mut buffer)
			.expect("failed to read file");
	}
	println!("{}", &buffer);
}