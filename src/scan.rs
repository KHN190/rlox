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
pub struct Token<'a> {
    pub tt: TokenType,
    // avoid local String is freed before return
    pub value: Cow<'a, str>,
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

    #[rustfmt::skip]
    pub fn next(&mut self) -> Token {
        self.skip_white_space();

        if self.at_end() {
            return token!(self, "EOF", TokenType::Eof);
        }
        self.current += 1;

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

            '"' => self.string(),

            '0'..='9' => self.number(),
            'A'..='Z' | 'a'..='z' | '_' => self.identifier(),

            _ => token!(self, "UNK", TokenType::Error),
        }
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    // see if next char matches expect
    fn expect(&mut self, expect: char) -> bool {
        if expect == '\0' || self.peek() != expect {
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
                '\n' | '\0' => {
                    self.line += 1;
                    self.current += 1;
                    break;
                },
                _ => self.current += 1,
            }
        }
    }

    // handle space
    fn skip_white_space(&mut self) {
        loop {
            let cur = self.peek();
            match cur {
            	'\r' | '\t' | ' ' => self.current += 1,
            	'\n' => {
            		self.line += 1;
            		self.current += 1;
            	},
                '/' => { if self.peek_next() == '/' { self.skip_comment(); } },
            	_ => break,
            }
        }
    }

    // handle string
    fn string(&mut self) -> Token {
        let start = self.current;

        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
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
        self.source[start..end].into_iter().collect::<String>()
    }

    // handle number
    fn number(&mut self) -> Token {
        let start = self.current - 1;

        while let '0'..='9' = self.peek() {
            self.current += 1; // for digit
        }
        if self.peek() == '.' && self._is_digit(self.peek_next()) {
            self.current += 1; // for '.'
            while let '0'..='9' = self.peek() {
                self.current += 1;
            }
        }
        let end = self.current;

        token!(self, self._string(start, end), TokenType::Number)
    }

    // handle identifier
    fn identifier(&mut self) -> Token {
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
    fn _is_alpha(&self, cur: char) -> bool {
        match cur {
            'A'..='Z' | 'a'..='z' | '_' => true,
            _ => false,
        }
    }

    fn _is_digit(&self, cur: char) -> bool {
        match cur {
            '0'..='9' => true,
            _ => false,
        }
    }
}
