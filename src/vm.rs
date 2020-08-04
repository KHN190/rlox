use crate::chunk::*;


#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum InterpretResult {
  OK,
  CompileError,
  RuntimeError,
} 


#[derive(Debug)]
pub struct VirtualMachine {
	// current op offset
	ip: usize,
}


impl VirtualMachine {
	pub fn new() -> VirtualMachine {
		VirtualMachine {
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
			let op = chunk.code[self.ip];
			match op  {
				Op::Return => { return InterpretResult::OK },
				_ => {},
			}
			self.ip += 1
		}
		panic!("Chunk has no return.");
	}
}