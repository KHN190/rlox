use crate::chunk::*;
use crate::debug::Disassembler;
use crate::DEBUG_TRACE;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum InterpretResult {
  OK,
  CompileError,
  RuntimeError,
} 


#[derive(Debug)]
pub struct VirtualMachine {
	// debug
	debugger: Disassembler,
	// current op offset
	ip: usize,
}


impl VirtualMachine {
	pub fn new() -> VirtualMachine {
		VirtualMachine {
			debugger: Disassembler::new(),
			ip: 0,
		}
	}

	// another possible way is to take the ownership,
	//
	// pub fn interpret(mut self, ref chunk: Chunk) { }

	pub fn interpret(mut self, chunk: &Chunk) -> InterpretResult {
		self.ip = 0;
  		self.run(chunk)
	}

	fn run(&mut self, chunk: &Chunk) -> InterpretResult {

		while self.ip < chunk.count {
			// print trace if debug is on
			if DEBUG_TRACE {
				self.debugger.disassemble_op(chunk);
			}
			// execute instruction
			let op = chunk.code[self.ip];
			match op  {
				Op::Return => {
					return InterpretResult::OK 
				},
				Op::Constant => {
					// let (idx, val) = chunk.get_constant(self.ip);
					self.ip += 2;
				},
				_ => {
					self.ip += 1
				},
			}
		}
		panic!("Chunk has no return.");
	}
}