use crate::chunk::{Chunk, Op};

macro_rules! line_info {
	( $self:expr, $bytes:expr ) => {
		if $self.offset > 0 && 
		   $bytes.get_line($self.offset) == $bytes.get_line($self.offset - 1) {
			String::from("    | ")
		} else {
			format!(" {:4} ", $bytes.get_line($self.offset))
		}
	}
}

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

	pub fn disassemble(mut self, ref bytes: Chunk, name: &str) {
		println!("== {} ==", name);
		println!("IDX | LINE | VALUE");

		// borrow the chunk using `&` or `ref`
		//   https://hellocode.dev/rust-ownership
		//   https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html

		while self.offset < bytes.count {
			self.disassemble_op(bytes);
		}
	}

	fn disassemble_op(&mut self, bytes: &Chunk) {
		print!("{:04} ", self.offset);

		// print line number
		let line = line_info!(self, bytes);
		print!("{}  ", line);

		let op = &bytes.code[self.offset];
		match op {
			Op::Return => {
				println!("OP_RETURN");
				self.offset += 1;
			},

			Op::Constant => {
				// parse Op to u8, usize
				let (idx, val) = bytes.get_constant(self.offset);

				println!("OP_CONSTANT  {:04} {}", idx, val);
				self.offset += 2;
			},

			_ => {
				println!("UNK");
				self.offset += 1;
			},
		}
	}
}
