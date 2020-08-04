use crate::chunk::*;

macro_rules! line_info {
	( $self:expr, $bytes:expr ) => {
		if $self.ip > 0 && 
		   $bytes.get_line($self.ip) == $bytes.get_line($self.ip - 1) {
			String::from("    | ")
		} else {
			format!(" {:4} ", $bytes.get_line($self.ip))
		}
	}
}

#[derive(Debug)]
pub struct Disassembler {
	ip: usize,
}

#[allow(dead_code)]
impl Disassembler {
	pub fn new() -> Self
	{
		Disassembler {
			ip: 0,
		}
	}

	pub fn disassemble(mut self, bytes: &Chunk, name: &str) {
		println!("== {} ==", name);
		println!("IDX | LINE | VALUE");

		// borrow the chunk using `&` or `ref`
		//   https://hellocode.dev/rust-ownership
		//   https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html

		while self.ip < bytes.count {
			self.disassemble_op(bytes);
		}
	}

	pub fn disassemble_op(&mut self, bytes: &Chunk) {
		print!("{:04} ", self.ip);

		// print line number
		let line = line_info!(self, bytes);
		print!("{}  ", line);

		let op = &bytes.code[self.ip];
		match op {
			Op::Return => {
				println!("OP_RETURN");
				self.ip += 1;
			},

			Op::Constant => {
				// parse Op to u8, usize
				let (idx, val) = bytes.get_constant(self.ip);

				println!("OP_CONSTANT  {:04} {}", idx, val);
				self.ip += 2;
			},

			_ => {
				println!("UNK");
				self.ip += 1;
			},
		}
	}
}
