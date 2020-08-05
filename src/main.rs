#[macro_use]
mod macros;

pub mod chunk;
pub mod debug;
pub mod vm;

#[cfg(test)]
mod tests;

pub const DEBUG_TRACE: bool = true;

use vm::VirtualMachine;
use chunk::*;

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
