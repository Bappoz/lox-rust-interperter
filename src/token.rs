#![allow(unused_variables)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEq,
    Eq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    // Literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    True,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Literals {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literals>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literals>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

