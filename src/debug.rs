use crate::chunk::*;

#[derive(Debug)]
pub struct Disassembler {
    ip: usize,
}

#[allow(dead_code)]
impl Disassembler {
    pub fn new() -> Self {
        Disassembler { ip: 0 }
    }

    pub fn disassemble(mut self, bytes: &Chunk, name: &str) {
        println!("== {} ==", name);
        println!("IDX | LINE | VALUE");

        // borrow the chunk using `&` or `ref`
        //   https://hellocode.dev/rust-ownership
        //   https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html

        while self.ip < bytes.count {
            self.disassemble_op(bytes);
        }
    }

    pub fn disassemble_op(&mut self, bytes: &Chunk) {
        print!("{:04} ", self.ip);

        // print line number
        let line = line_info!(self, bytes);
        print!("{}  ", line);

        let op = &bytes.code[self.ip];
        match op {
            Op::Return => {
                println!("OP_RETURN");
                self.ip += 1;
            }
            Op::Constant => {
                let (idx, val) = bytes.get_constant(self.ip);

                println!("OP_CONST  {:04} {}", idx, val);
                self.ip += 2;
            }
            Op::Negate => {
                println!("OP_NEG");
                self.ip += 1;
            }
            Op::Add => {
                println!("OP_ADD");
                self.ip += 1;
            }
            Op::Subtract => {
                println!("OP_SUB");
                self.ip += 1;
            }
            Op::Multiply => {
                println!("OP_MUL");
                self.ip += 1;
            }
            Op::Divide => {
                println!("OP_DIV");
                self.ip += 1;
            }
            _ => {
                println!("OP_UNK");
                self.ip += 1;
            }
        }
    }
}
