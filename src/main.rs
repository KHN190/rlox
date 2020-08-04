#[macro_use]
mod memory;
mod chunk;
mod vm;
mod debug;

#[allow(unused_imports)]
use debug::{Disassembler};
#[allow(unused_imports)]
use vm::{VirtualMachine};

use chunk::*;

fn main() {
	let mut chk = Chunk::new();

	let constant = chk.add_constant(1.2);
	chk.write(Op::Constant, 0);
	chk.write(constant, 0);

	chk.write(Op::Return, 1);
	chk.write(Op::Nil, 1);

	// let debugger = Disassembler::new();
	// debugger.disassemble(chk, "rlox VM");

	let vm = VirtualMachine::new();
	vm.interpret(chk);
}
