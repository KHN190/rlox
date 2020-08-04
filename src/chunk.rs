pub type Value = f64;

#[repr(u8)]
#[derive(Debug)]
pub enum Op {
	// default type is Chunk::Op
	Constant,
	ConstantIndex(u8),
	Return,
	Nil,
}

#[derive(Debug)]
pub struct Chunk {
	// Rust array can only be created using 
	// compile time known length, thus
	// you can't do sth like: let code = [0; len];
	pub code: Vec<Op>,
	pub constants: Vec<Value>,
	// Line information for error report
	pub lines: Vec<usize>,
	pub count: usize,
	pub capacity: usize,
}

impl Chunk {

	pub fn new() -> Chunk {
		Chunk {
			code: Vec::new(),
			constants: Vec::new(),
			lines: Vec::new(),
			count: 0,
			capacity: 0,
		}
	}

	// let mut chunk = Chunk::new();
	// chunk.write(Op::Return);
	pub fn write(&mut self, byte: Op, line: usize) {
		if self.capacity < self.count + 1 {
			self.capacity = grow_capacity!(self.capacity);
		}

		self.code.push(byte);
		self.lines.push(line);
		self.count += 1;
	}

	// return constant index
	pub fn add_constant(&mut self, value: Value) -> Op {
		if self.constants.len() >= 256 {
			panic!("A chunk cannot have more than 256 constants");
		}
		self.constants.push(value);
		Op::ConstantIndex(
			self.constants.len() as u8 - 1)
	}
}
