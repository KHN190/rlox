// Many languages split into two separate passes.
// A parser produces an AST and then a “code generator” traverses the
// AST and outputs target code. In rlox, we’re merging these two passes
// into one.

use std::option::Option;

use crate::chunk::*;
use crate::lexer::*;

#[derive(Debug)]
pub struct Parser<'src> {
    curr: Option<Token<'src>>,
    prev: Option<Token<'src>>,
    scanner: &'src mut Scanner<'src>,
    pub had_error: bool,
    panic: bool,
}

impl<'src> Parser<'src> {
    pub fn new(scanner: &'src mut Scanner<'src>) -> Parser<'src> {
        Parser {
            curr: None,
            prev: None,
            scanner,
            had_error: false,
            panic: false,
        }
    }

    pub fn advance(&mut self) {
        self.prev = self.curr.take();
        self.curr = Some(self.scanner.next());

        if let TokenType::Error = self.curr_token().tt {
            self.lexic_error();
        }
    }

    pub fn curr_token(&self) -> &Token {
        self.curr.as_ref().unwrap()
    }

    pub fn prev_token(&self) -> &Token {
        self.prev.as_ref().unwrap()
    }

    fn emit_byte(&mut self, op: Op, bytes: &mut Chunk) {
    	bytes.write(op, self.prev_token().line);
    }

    fn emit_return(&mut self, bytes: &mut Chunk) {
    	self.emit_byte(Op::Return, bytes);
    }

    pub fn end_compile(&mut self, bytes: &mut Chunk) {
    	println!("prev: {:?}, curr: {:?}", self.prev, self.curr);

    	self.emit_return(bytes);
    }
}

/* Parse Syntax */
pub trait SyntaxTrait {
    fn expression(&mut self);
    fn consume(&mut self, tt: TokenType, msg: &str);
}

impl<'src> SyntaxTrait for Parser<'src> {
    fn expression(&mut self) {
        println!("parse expression.");
    }

    fn consume(&mut self, tt: TokenType, msg: &str) {
        if self.curr_token().tt == tt {
            self.advance();
            return;
        }
        self.syntax_error(msg);
    }
}

/* Error Report */
trait ErrorReportTrait {
	fn lexic_error(&mut self);
	fn syntax_error(&mut self, msg: &str);
}

impl<'src> ErrorReportTrait for Parser<'src> {

    fn lexic_error(&mut self) {
        self.syntax_error("Unexpected token.");
    }

    fn syntax_error(&mut self, msg: &str) {
        if self.panic {
            return;
        }
        self.panic = true;

        let token = self.curr_token();
        println!("line[{}] error at token: {:?} {}", token.line, token.tt, token.value);
        println!("{}", msg);

        self.had_error = true;
    }
}
