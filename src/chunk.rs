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
	pub count: usize,
	pub capacity: usize,
}

impl Chunk {

	pub fn new() -> Chunk {
		Chunk {
			code: Vec::new(),
			constants: Vec::new(),
			count: 0,
			capacity: 0,
		}
	}

	// let mut chunk = Chunk::new();
	// chunk.write(8);
	pub fn write(&mut self, byte: Op) {
		if self.capacity < self.count + 1 {
			self.capacity = grow_capacity!(self.capacity);
		}

		self.code.push(byte);
		self.count += 1;
	}

	// return constant index
	pub fn add_constant(&mut self, value: Value) -> u8 {
		if self.constants.len() >= 256 {
			panic!("A chunk cannot have more than 256 constants");
		}
		self.constants.push(value);
		self.constants.len() as u8 - 1
	}
}
