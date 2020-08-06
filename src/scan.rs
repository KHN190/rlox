macro_rules! token {
    ( $self:ident, $val:literal, $tt:expr ) => {
        // Token::new($tt, $val, $self.line)
        Token {
            tt: $tt,
            value: $val,
            line: $self.line,
        }
    };
}

#[rustfmt::skip]
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
	// Single-character tokens.
	LeftParen, RightParen,
	LeftBrace, RightBrace,
	Comma, Dot, Minus, Plus,
	Semicolon, Slash, Star,

	// One or two character tokens.
	Bang, BangEqual,
	Equal, EqualEqual,
	Greater, GreaterEqual,
	Less, LessEqual,

	// Literals.
	Identifier, Str, Number,

	// Keywords.
	And, Class, Else, False,
	For, Fun, If, Nil, Or,
	Peint, Return, Super, This,
	True, Var, While,

	Error, Eof
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub tt: TokenType,
    pub value: &'a str,
    pub line: usize,
}

#[derive(Debug)]
pub struct Scanner {
    pub source: Vec<char>,
    pub current: usize,
    pub line: usize,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner::new("")
    }
}

impl Scanner {
    pub fn new(text: &str) -> Scanner {
        Scanner {
            source: text.chars().collect(),
            current: 0,
            line: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.current += 1;

        if self.current > self.source.len() {
            return token!(self, "EOF", TokenType::Eof);
        } else {
            let cur = self.source[self.current - 1];
            match cur {
                '(' => token!(self, "(", TokenType::LeftParen),
                ')' => token!(self, ")", TokenType::RightParen),
                _ => token!(self, "UNK", TokenType::Error),
            }
        }
    }

    pub fn peek(&mut self) -> char {
        panic!("not implemented");
    }
}
