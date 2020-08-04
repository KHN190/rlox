#[macro_use]
mod memory;
mod chunk;
mod debug;

use chunk::*;
use debug::{Disassembler};

fn main() {
	let debugger = Disassembler::new();

	let mut chk = Chunk::new();

	let constant = chk.add_constant(1.2);
	chk.write(Op::Constant, 0);
	chk.write(constant, 0);

	chk.write(Op::Return, 1);
	chk.write(Op::Nil, 1);

	debugger.disassemble(chk, "rlox VM");
}
