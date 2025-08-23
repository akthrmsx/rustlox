#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: Kind,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new<A: Into<String>>(kind: Kind, lexeme: A, literal: Literal, line: usize) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Self {
            kind: Kind::Eof,
            lexeme: "".into(),
            literal: Literal::None,
            line,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Assign,
    Bang,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Identifier,
    String,
    Number,
    True,
    False,
    Nil,
    And,
    Or,
    If,
    Else,
    For,
    While,
    Class,
    This,
    Super,
    Fun,
    Return,
    Var,
    Print,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    None,
    Identifier(String),
    String(String),
    Number(f64),
}

impl Literal {
    pub fn none() -> Self {
        Self::None
    }

    pub fn identifier<A: Into<String>>(value: A) -> Self {
        Self::Identifier(value.into())
    }

    pub fn string<A: Into<String>>(value: A) -> Self {
        Self::String(value.into())
    }

    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }
}
