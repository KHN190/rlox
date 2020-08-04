use crate::chunk::*;
use crate::debug::Disassembler;
use crate::vm::*;


#[test]
fn test_write_constant() {
	let mut chk = Chunk::new();
	let constant = chk.add_constant(1.0);
	chk.write(Op::Constant, 0);
	chk.write(constant, 0);

	assert_eq!(chk.count, 2);
	assert_eq!(chk.code[0], Op::Constant);

	let (idx, val) = chk.get_constant(0);
	assert_eq!(idx, 0);
	assert_eq!(val, 1.0);
}

#[test]
fn test_debugger() {
	let mut chk = Chunk::new();
	chk.write(Op::Nil, 0);

	let debugger = Disassembler::new();
	debugger.disassemble(&chk, "unit test");

	assert_eq!(chk.count, 1);
	assert_eq!(chk.code[0], Op::Nil);
}

#[test]
fn test_vm() {
	let mut chk = Chunk::new();
	chk.write(Op::Nil, 0);
	chk.write(Op::Return, 1);

	let vm = VirtualMachine::new();
	let code = vm.interpret(&chk);

	assert_eq!(chk.count, 2);
	assert_eq!(chk.code[1], Op::Return);
	assert_eq!(code, InterpretResult::OK);
}

#[test]
fn test_clear_chunk() {
	let mut chk = Chunk::new();
	chk.add_constant(1.0);
	chk.clear();

	assert_eq!(chk.code.len(), 0);
	assert_eq!(chk.count, 0);
}