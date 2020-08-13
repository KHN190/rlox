use std::borrow::Cow;

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
	Print, Return, Super, This,
	True, Var, While,

	Error, Eof
}

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub tt: TokenType,
    // avoid local String is freed before return
    pub value: Cow<'src, str>,
    pub line: usize,
}

#[derive(Debug)]
pub struct Scanner<'src> {
    pub source: &'src [u8],
    pub current: usize,
    pub line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(text: &'src str) -> Scanner<'src> {
        Scanner {
            source: text.as_bytes(),
            current: 0,
            line: 0,
        }
    }

    #[rustfmt::skip]
    pub fn next(&mut self) -> Token<'static> {
        self.skip_white_space();

        if self.at_end() {
            return token!(self, "EOF", TokenType::Eof);
        }
        self.current += 1;

        let cur = self.source[self.current - 1];
        match cur {
            b'(' => token!(self, "(", TokenType::LeftParen),
            b')' => token!(self, ")", TokenType::RightParen),
            b'{' => token!(self, "{", TokenType::LeftBrace),
            b'}' => token!(self, "}", TokenType::RightBrace),
            b';' => token!(self, ";", TokenType::Semicolon),
            b',' => token!(self, ",", TokenType::Comma),
            b'.' => token!(self, ".", TokenType::Dot),
            b'-' => token!(self, "-", TokenType::Minus),
            b'+' => token!(self, "+", TokenType::Plus),
            b'/' => token!(self, "/", TokenType::Slash),
            b'*' => token!(self, "*", TokenType::Star),

            b'!' => if_then_token!(self, b'=', "!=", TokenType::BangEqual, "!", TokenType::Bang),
            b'=' => if_then_token!(self, b'=', "==", TokenType::EqualEqual, "=", TokenType::Equal),
            b'<' => if_then_token!(self, b'=', "<=", TokenType::LessEqual, "<", TokenType::Less),
            b'>' => if_then_token!(self, b'=', ">=", TokenType::GreaterEqual, ">", TokenType::Greater),

            b'"' => self.string(),

            b'0'..=b'9' => self.number(),
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.identifier(),

            _ => token!(self, "UNK", TokenType::Error),
        }
    }

    fn peek(&self) -> u8 {
        if self.at_end() {
            return b'\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        self.source[self.current + 1]
    }

    // see if next char matches expect
    fn expect(&mut self, expect: u8) -> bool {
        if expect == b'\0' || self.peek() != expect {
            return false;
        }
        self.current += 1;
        true
    }

    // see if cur char is EOF
    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // handle comment
    fn skip_comment(&mut self) {
        loop {
            let cur = self.peek();
            match cur {
                b'\n' | b'\0' => {
                    self.line += 1;
                    self.current += 1;
                    break;
                }
                _ => self.current += 1,
            }
        }
    }

    // handle space
    fn skip_white_space(&mut self) {
        loop {
            let cur = self.peek();
            match cur {
                b'\r' | b'\t' | b' ' => self.current += 1,
                b'\n' => {
                    self.line += 1;
                    self.current += 1;
                }
                b'/' => {
                    if self.peek_next() != b'/' {
                        break;
                    }
                    self.skip_comment();
                }
                _ => break,
            }
        }
    }

    // handle string
    fn string(&mut self) -> Token<'static> {
        let start = self.current;

        while self.peek() != b'"' && !self.at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.at_end() {
            return token!(self, "Unterminated string", TokenType::Error);
        }
        let end = self.current;
        // enclosing string
        self.current += 1;

        token!(self, self._string(start, end), TokenType::Str)
    }

    fn _string(&self, start: usize, end: usize) -> String {
        // self.source[start..end].iter().collect::<String>()
        String::from(std::str::from_utf8(&self.source[start..end]).unwrap())
    }

    // handle number
    fn number(&mut self) -> Token<'static> {
        let start = self.current - 1;

        while self._is_digit(self.peek()) {
            self.current += 1; // for digit
        }
        if self.peek() == b'.' && self._is_digit(self.peek_next()) {
            self.current += 1; // for b'.'
            while self._is_digit(self.peek()) {
                self.current += 1;
            }
        }
        let end = self.current;

        token!(self, self._string(start, end), TokenType::Number)
    }

    // handle identifier
    fn identifier(&mut self) -> Token<'static> {
        let start = self.current - 1;

        while self._is_alpha(self.peek()) || self._is_digit(self.peek()) {
            self.current += 1;
        }
        let end = self.current;

        let val = self._string(start, end);
        let typ = self._identifier_type(&val);

        token!(self, val, typ)
    }

    // handle keyword
    fn _identifier_type(&self, id: &str) -> TokenType {
        match id {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,

            _ => TokenType::Identifier,
        }
    }

    // utils
    fn _is_alpha(&self, cur: u8) -> bool {
        match cur {
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => true,
            _ => false,
        }
    }

    fn _is_digit(&self, cur: u8) -> bool {
        match cur {
            b'0'..=b'9' => true,
            _ => false,
        }
    }
}
