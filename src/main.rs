#[macro_use]
mod memory;
mod chunk;
mod vm;
mod debug;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
use debug::{Disassembler};
#[allow(unused_imports)]
use vm::{VirtualMachine};

use chunk::*;

pub const DEBUG_TRACE: bool = true;

fn main() {
	let mut chk = Chunk::new();

	let constant = chk.add_constant(1.2);
	chk.write(Op::Constant, 0);
	chk.write(constant, 0);

	chk.write(Op::Nil, 1);
	chk.write(Op::Negate, 1);
	chk.write(Op::Return, 1);

	let vm = VirtualMachine::new();
	vm.interpret(&chk);
}
