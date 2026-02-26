use crate::token::TokenType::Eof;
use crate::token::{Literals, Token, TokenType};

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

        while !self.is_at_end() {
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

        if !errors.is_empty() {
            let mut join = "".to_string();
            let _ = errors.iter().map(|msg| {
                join.push_str(msg);
                join.push('\n')
            });
            return Err(join);
        }
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        (c as u8) >= '0' as u8 && (c as u8) <= '9' as u8
    }

    // CHANGE RETURN TYPE TO 'TOKEN'
    fn scan_token(&mut self) -> Result<(), String> {
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
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            '!' => {
                let token = if self.matching('=') {
                    TokenType::BangEq
                } else {
                    TokenType::Bang
                };
                self.add_token(token)
            }

            '=' => {
                let token = if self.matching('=') {
                    TokenType::EqEq
                } else {
                    TokenType::Eq
                };
                self.add_token(token)
            }

            '<' => {
                let token = if self.matching('=') {
                    TokenType::LessEq
                } else {
                    TokenType::Less
                };
                self.add_token(token)
            }

            '>' => {
                let token = if self.matching('=') {
                    TokenType::GreaterEq
                } else {
                    TokenType::Greater
                };
                self.add_token(token)
            }
            '/' => {
                if self.matching('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            c => {
                if self.is_digit(c) {
                    match self.number() {
                        Ok(_) => {}
                        _ => return Err(format!("Unknown char '{}', at line {}: ", c, self.line)),
                    }
                }
            }
        }
        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let substring = &self.source[self.start..self.current];
        match substring.parse::<f64>() {
            Ok(value) => self.add_token_literal(TokenType::Number, Some(Literals::Float(value))),
            Err(_) => return Err("Failed to Parse.".to_string()),
        };

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated String".to_string());
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token_literal(TokenType::String, Some(Literals::String(value.to_string())));
        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn matching(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literals>) {
        let mut text = "".to_string();
        let _lit = self.source[self.start..self.current]
            .chars()
            .map(|c| text.push(c));

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn handle_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[2].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[3].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_two_chars_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Bang);
        assert_eq!(scanner.tokens[1].token_type, TokenType::BangEq);
        assert_eq!(scanner.tokens[2].token_type, TokenType::EqEq);
        assert_eq!(scanner.tokens[3].token_type, TokenType::GreaterEq);
        assert_eq!(scanner.tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_string_literals() {
        let source = r#""TEST""#;
        print!("{}", source);
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::String);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            Literals::String(val) => assert_eq!(val, "TEST"),
            _ => panic!("Incorrect literal!"),
        }
    }

    #[test]
    fn handle_string_lit_multiple_lines() {
        let source = "\"TEST\nnewline\"";
        print!("{}", source);
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::String);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            Literals::String(val) => assert_eq!(val, "TEST\nnewline"),
            _ => panic!("Incorrect literal!"),
        }
    }

    #[test]
    fn handle_number_lit() {
        let source = "123 123.456";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            Literals::Float(val) => assert_eq!(*val, 123.0),
            _ => panic!("Incorrect literal!"),
        }

        assert_eq!(scanner.tokens[1].token_type, TokenType::Number);
        match scanner.tokens[1].literal.as_ref().unwrap() {
            Literals::Float(val) => assert_eq!(*val, 123.456),
            _ => panic!("Incorrect literal!"),
        }
    }
}
