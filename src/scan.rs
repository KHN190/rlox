macro_rules! token {
    ( $self:ident, $val:literal, $tt:expr ) => {
        Token {
            tt: $tt,
            value: $val,
            line: $self.line,
        }
    };
}

macro_rules! if_then_token {
    ( $self:ident, $if:literal, $then_val:literal, $then:expr, $else_val:literal, $else:expr ) => {
        if $self.expect($if) {
            token!($self, $then_val, $then)
        } else {
            token!($self, $else_val, $else)
        }
    };
}

#[rustfmt::skip]
#[allow(dead_code)]
#[repr(u16)]
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
        self.skip_white_space();

        self.current += 1;
        if self.current > self.source.len() {
            return token!(self, "EOF", TokenType::Eof);
        } else {
            let cur = self.source[self.current - 1];
            match cur {
                '(' => token!(self, "(", TokenType::LeftParen),
                ')' => token!(self, ")", TokenType::RightParen),
                '{' => token!(self, "{", TokenType::LeftBrace),
                '}' => token!(self, "}", TokenType::RightBrace),
                ';' => token!(self, ";", TokenType::Semicolon),
                ',' => token!(self, ",", TokenType::Comma),
                '.' => token!(self, ".", TokenType::Dot),
                '-' => token!(self, "-", TokenType::Minus),
                '+' => token!(self, "+", TokenType::Plus),
                '/' => token!(self, "/", TokenType::Slash),
                '*' => token!(self, "*", TokenType::Star),

                '!' => if_then_token!(self, '=', "!=", TokenType::BangEqual, "!", TokenType::Bang),
                '=' => if_then_token!(self, '=', "==", TokenType::EqualEqual, "=", TokenType::Equal),
                '<' => if_then_token!(self, '=', "<=", TokenType::LessEqual, "<", TokenType::Less),
                '>' => if_then_token!(self, '=', ">=", TokenType::GreaterEqual, ">", TokenType::Greater),

                _ => token!(self, "UNK", TokenType::Error),
            }
        }
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        self.source[self.current]
    }

    // see if next char matches expect
    fn expect(&mut self, expect: char) -> bool {
        if expect == '\0' || self.peek() != expect {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_white_space(&mut self) {
        loop {
            let cur = self.peek();
            if cur == '\r' || cur == '\t' || cur == ' ' {
                self.current += 1;
            } else {
                break;
            }
        }
    }
}
