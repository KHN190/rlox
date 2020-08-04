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
	pub count: usize,
	pub capacity: usize,
	// Line information for error report
	// compressed with run-length encoding
	lines: Vec<usize>,
	// store constants
	constants: Vec<Value>,
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
		self.write_line(line);
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

	// return constant index & value
	pub fn get_constant(& self, offset: usize) -> (u8, Value) {
		if let Op::ConstantIndex(idx) = self.code[offset + 1] {
			return (idx, self.constants[idx as usize]);
		}
		panic!("Cannot access constant");
	}

	// run length encoding
	// 1,1,1,3,3 -> 1,3,3,2
	fn write_line(&mut self, ln: usize) {
		let len = self.lines.len();
		if len == 0 {
			self.lines.push(ln);
			self.lines.push(1);

		} else {
			let last_ln = self.lines[len - 2];
			if ln == last_ln { 
				self.lines[len - 1] += 1;
			} else {
				self.lines.push(ln);
				self.lines.push(1);
			}	
		}
	}

	// run length decoding
	pub fn get_line(& self, offset: usize) -> usize {
		let len = self.lines.len();
		let mut i = 0;
		let mut total_op = 0;
		while i < len {
			total_op += self.lines[i + 1];
			if total_op > offset { return self.lines[i]; }
			i += 2;
		}
		return 0;
	}
}
