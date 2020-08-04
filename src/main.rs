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
	let vm = VirtualMachine::new();

	vm.interpret(&chk);
}
