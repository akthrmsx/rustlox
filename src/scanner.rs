use crate::{
    error::ScanError,
    token::{Kind, Literal, Token},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    errors: Vec<ScanError>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new<A: Into<String>>(source: A) -> Self {
        Self {
            source: source.into().chars().collect(),
            tokens: vec![],
            errors: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<ScanError>> {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        self.tokens.push(Token::eof(self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => {
                self.append_token(Kind::LeftParen);
            }
            ')' => {
                self.append_token(Kind::RightParen);
            }
            '{' => {
                self.append_token(Kind::LeftBrace);
            }
            '}' => {
                self.append_token(Kind::RightBrace);
            }
            '.' => {
                self.append_token(Kind::Dot);
            }
            ',' => {
                self.append_token(Kind::Comma);
            }
            ';' => {
                self.append_token(Kind::Semicolon);
            }
            '+' => {
                self.append_token(Kind::Plus);
            }
            '-' => {
                self.append_token(Kind::Minus);
            }
            '*' => {
                self.append_token(Kind::Asterisk);
            }
            '/' => {
                if self.match_current_char('/') {
                    while !self.is_current_char('\n') && !self.is_eof() {
                        self.advance();
                    }
                } else {
                    self.append_token(Kind::Slash);
                }
            }
            '=' => {
                if self.match_current_char('=') {
                    self.append_token(Kind::Eq);
                } else {
                    self.append_token(Kind::Assign);
                }
            }
            '!' => {
                if self.match_current_char('=') {
                    self.append_token(Kind::Ne);
                } else {
                    self.append_token(Kind::Bang);
                }
            }
            '<' => {
                if self.match_current_char('=') {
                    self.append_token(Kind::Le);
                } else {
                    self.append_token(Kind::Lt);
                }
            }
            '>' => {
                if self.match_current_char('=') {
                    self.append_token(Kind::Ge);
                } else {
                    self.append_token(Kind::Gt);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.scan_token_string();
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                self.scan_token_keyword_or_identifier();
            }
            c if c.is_ascii_digit() => {
                self.scan_token_number();
            }
            _ => {
                self.append_error("unexpected character.");
            }
        }
    }

    fn scan_token_keyword_or_identifier(&mut self) {
        while self.get_current_char().is_ascii_alphanumeric() || self.is_current_char('_') {
            self.advance();
        }

        match self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .as_str()
        {
            "true" => {
                self.append_token(Kind::True);
            }
            "false" => {
                self.append_token(Kind::False);
            }
            "nil" => {
                self.append_token(Kind::Nil);
            }
            "and" => {
                self.append_token(Kind::And);
            }
            "or" => {
                self.append_token(Kind::Or);
            }
            "if" => {
                self.append_token(Kind::If);
            }
            "else" => {
                self.append_token(Kind::Else);
            }
            "for" => {
                self.append_token(Kind::For);
            }
            "while" => {
                self.append_token(Kind::While);
            }
            "class" => {
                self.append_token(Kind::Class);
            }
            "this" => {
                self.append_token(Kind::This);
            }
            "super" => {
                self.append_token(Kind::Super);
            }
            "fun" => {
                self.append_token(Kind::Fun);
            }
            "return" => {
                self.append_token(Kind::Return);
            }
            "var" => {
                self.append_token(Kind::Var);
            }
            "print" => {
                self.append_token(Kind::Print);
            }
            identifier => {
                self.append_token_with_literal(Kind::Identifier, Literal::identifier(identifier));
            }
        }
    }

    fn scan_token_string(&mut self) {
        while !self.is_current_char('"') && !self.is_eof() {
            if self.is_current_char('\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_eof() {
            self.append_error("unterminated string.");
            return;
        }

        self.advance();

        self.append_token_with_literal(
            Kind::String,
            Literal::string(
                self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect::<String>(),
            ),
        );
    }

    fn scan_token_number(&mut self) {
        while self.get_current_char().is_ascii_digit() {
            self.advance();
        }

        if self.is_current_char('.') && self.get_next_char().is_ascii_digit() {
            self.advance();

            while self.get_current_char().is_ascii_digit() {
                self.advance();
            }
        }

        self.append_token_with_literal(
            Kind::Number,
            Literal::number(
                self.source[self.start..self.current]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            ),
        );
    }

    fn append_token(&mut self, kind: Kind) {
        self.append_token_with_literal(kind, Literal::none());
    }

    fn append_token_with_literal(&mut self, kind: Kind, literal: Literal) {
        self.tokens.push(Token::new(
            kind,
            self.source[self.start..self.current]
                .iter()
                .collect::<String>(),
            literal,
            self.line,
        ));
    }

    fn append_error(&mut self, message: &str) {
        self.errors.push(ScanError::new(message, self.line));
    }

    fn advance(&mut self) -> char {
        let c = self.get_current_char();
        self.current += 1;
        c
    }

    fn get_current_char(&self) -> char {
        if self.is_eof() {
            return '\0';
        }

        self.source[self.current]
    }

    fn get_next_char(&self) -> char {
        let next = self.current + 1;

        if next >= self.source.len() {
            return '\0';
        }

        self.source[next]
    }

    fn match_current_char(&mut self, c: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if !self.is_current_char(c) {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_current_char(&self, c: char) -> bool {
        self.get_current_char() == c
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }
}
