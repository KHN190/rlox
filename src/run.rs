// use crate::vm::VirtualMachine;

use crate::scan::{Scanner, TokenType};
use crate::vm::InterpretResult;

// let vm = VirtualMachine::new();
// let scanner = Scanner::default();

#[allow(dead_code)]
pub fn repl() {
    use rustyline::error::ReadlineError;
    use rustyline::Editor;

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                interpret(&line);
                println!("{}", line);
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

#[allow(dead_code)]
pub fn run_file(filename: &str) {
    use std::fs::File;
    use std::io::Read;

    println!("running file: {}", filename);

    let mut buffer = String::new();
    {
        File::open(&filename)
            .expect("failed to open file")
            .read_to_string(&mut buffer)
            .expect("failed to read file");
    }
    let status = interpret(&buffer);
    match status {
        InterpretResult::OK => {}
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
    }
}

fn interpret(source: &str) -> InterpretResult {
    compile(source);
    InterpretResult::OK
}

fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line = usize::MAX;
    loop {
        let token = scanner.next();
        if token.line != line {
            print!("[line {:4}] ", token.line);
            line = token.line;
        } else {
            print!("          | ");
        }
        println!("{:?} {}", token.tt, token.value);

        if let TokenType::Eof = token.tt {
            break;
        }
    }
}
