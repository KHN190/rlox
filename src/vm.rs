use crate::chunk::*;
use crate::debug::Disassembler;
use crate::DEBUG_TRACE;

const STACK_SIZE: usize = 4098;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretResult {
    OK,
    CompileError,
    RuntimeError,
}

#[derive(Debug, Default)]
pub struct VirtualMachine {
    // debug
    debugger: Disassembler,
    // current op offset
    ip: usize,
    // store unfinished op
    stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            debugger: Disassembler::new(),
            ip: 0,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    // another possible way is to take the ownership,
    //
    // pub fn interpret(mut self, ref bytes: Chunk) { }

    pub fn interpret(mut self, bytes: &Chunk) -> InterpretResult {
        self.ip = 0;
        self.run(bytes)
    }

    fn run(&mut self, bytes: &Chunk) -> InterpretResult {
        while self.ip < bytes.count {
            // print trace if debug is on
            if DEBUG_TRACE {
                // print stack
                for val in self.stack.iter() {
                    print!("[{}]", val);
                }
                println!();
                // print op
                self.debugger.disassemble_op(bytes);
            }
            // execute instruction
            let op = bytes.code[self.ip];
            match op {
                Op::Return => {
                    let val = self.stack.pop().unwrap();

                    println!("{}", val);
                    return InterpretResult::OK;
                }
                Op::Constant => {
                    let (_, val) = bytes.get_constant(self.ip);
                    self.stack.push(val);
                    self.ip += 2;
                }
                Op::Negate => {
                    unary_op!(self, -);
                }
                Op::Add => {
                    bin_op!(self, +);
                }
                Op::Subtract => {
                    bin_op!(self, -);
                }
                Op::Multiply => {
                    bin_op!(self, *);
                }
                Op::Divide => {
                    bin_op!(self, /);
                }
                _ => self.ip += 1,
            }
        }
        panic!("Chunk has no return.");
    }
}
