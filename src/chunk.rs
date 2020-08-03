#[repr(u8)]
#[derive(Clone)]
pub enum Op {
	// default type is Chunk::Op
	Return,
	Nil,
}

#[derive(Clone)]
pub struct Chunk {
	// Rust array can only be created using 
	// compile time known length, thus
	// you can't do sth like: let code = [0; len];
	pub code: Vec<Op>,
	pub count: usize,
	pub capacity: usize,
}

impl Chunk {

	pub fn new() -> Chunk {
		Chunk {
			code: Vec::new(),
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
}
