use crate::chunk::{Chunk, Op};

#[derive(Debug)]
pub struct Disassembler {
	offset: usize,
}

impl Disassembler {
	pub fn new() -> Self
	{
		Disassembler {
			offset: 0,
		}
	}

	pub fn disassemble(mut self, chunk: &Chunk, name: &str) {
		println!("== {} ==", name);

		// borrow the chunk
		//   https://hellocode.dev/rust-ownership
		let bytes = chunk;

		while self.offset < bytes.code.len() {
			self.disassemble_op(bytes);
		}
	}

	pub fn disassemble_op(&mut self, bytes: &Chunk) {
		print!("{:04} ", self.offset);

		let op = &bytes.code[self.offset];
		match op {
			Op::Return => {
				println!("OP_RETURN");
				self.offset += 1;
			},
			_ => {
				println!("UNK");
				self.offset += 1;
			},
		}
	}
}
