#[macro_use]
mod memory;
mod chunk;
mod debug;

// use chunk::*;
use chunk::{Chunk, Op};
use debug::{Disassembler};

fn main() {
	let debugger = Disassembler::new();

	let mut chk = Chunk::new();

	let constant = Op::ConstantIndex(chk.add_constant(1.2));
	chk.write(Op::Constant);
	chk.write(constant);

	chk.write(Op::Return);
	chk.write(Op::Nil);

	debugger.disassemble(chk, "rlox VM");
}
