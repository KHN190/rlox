// Many languages split into two separate passes.
// A parser produces an AST and then a “code generator” traverses the
// AST and outputs target code. In rlox, we’re merging these two passes
// into one.

use std::option::Option;

use crate::lexer::*;

#[derive(Debug)]
pub struct Parser<'src> {
    pub curr: Option<Token<'src>>,
    pub prev: Option<Token<'src>>,
    scanner: &'src mut Scanner<'src>,
}

impl<'src> Parser<'src> {
    pub fn new(scanner: &'src mut Scanner<'src>) -> Parser<'src> {
        Parser {
            curr: None,
            prev: None,
            scanner,
        }
    }

    pub fn advance(&mut self) {
        self.prev = self.curr.take();
        self.curr = Some(self.scanner.next());

        if let TokenType::Error = self.curr.as_ref().unwrap().tt {
            self.error();
        }
    }

    fn error(&self) {
        let token = &self.curr.as_ref().unwrap();
        println!("error at token: {:?} {}", token.tt, token.value);
    }
}
