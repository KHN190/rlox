use crate::DEBUG_LEXER;

use crate::chunk::Chunk;
use crate::compiler::*;
use crate::lexer::*;
use crate::vm::*;

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
    let mut chk = Chunk::new();

    if !compile(source, &mut chk) {
        return InterpretResult::CompileError;
    }
    VirtualMachine::new().interpret(&chk)
}

fn compile(source: &str, bytes: &mut Chunk) -> bool {
    let mut line = usize::MAX;
    let mut no_err = true;

    if DEBUG_LEXER {
        loop {
            let mut scanner = Scanner::new(source);
            let token = scanner.next();

            if token.line != line {
                print!("[line {:4}] ", token.line);
                line = token.line;
            } else {
                print!("          | ");
            }
            println!("{:?} {}", token.tt, token.value);

            if let TokenType::Error = token.tt {
                no_err = false;
            }
            if let TokenType::Eof = token.tt {
                break;
            }
        }

    } else {
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new(&mut scanner);

        parser.advance();
        parser.expression();
        parser.consume(TokenType::Eof, "Expect end of expression.");
        
        if parser.had_error {
            no_err = false;
        } else {
            parser.end_compile(bytes);
        }
    }
    !no_err
}
