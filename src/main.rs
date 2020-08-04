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

	let c1 = chk.add_constant(1.2);
	chk.write(Op::Constant, 0);
	chk.write(c1, 0);

	let c2 = chk.add_constant(3.4);
	chk.write(Op::Constant, 1);
	chk.write(c2, 1);

	chk.write(Op::Add, 2);

	let c3 = chk.add_constant(5.6);
	chk.write(Op::Constant, 2);
	chk.write(c3, 2);

	chk.write(Op::Divide, 3);

	chk.write(Op::Negate, 4);
	chk.write(Op::Return, 4);

	let vm = VirtualMachine::new();
	vm.interpret(&chk);
}
