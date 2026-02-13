use crate::token::{Literals, Token, TokenType};
use crate::token::TokenType::Eof;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while (!self.is_at_end()) {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }
        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut join = "".to_string();
            errors.iter().map(|msg| {
                join.push_str(msg);
                join.push_str("\n")
            });
            return Err(join);
        }
        Ok(self.tokens.clone())
    }



    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) -> Result<Token, String> {
        let c = self.advance();


        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => return Err(format!("Unknown char '{}', at line {}: ", c, self.line)),
        }
        todo!()
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_tokens(token_type, None);
    }

    fn add_tokens(
        &mut self,
        token_type: TokenType,
        literal: Option<Literals>,
    ) {
        let mut text = "".to_string();
        let bytes = self.source.as_bytes();
        for i in self.start..self.current {
            text.push(bytes[i] as char)
        }

        self.tokens.push( Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}
