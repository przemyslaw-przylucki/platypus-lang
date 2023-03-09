use std::collections::HashMap;
use std::iter::Iterator;
use crate::exception::Exception;
use crate::literal_value::LiteralValue;
use crate::literal_value::LiteralValue::{FloatValue, StringValue};
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("&&", TokenType::And),
                ("or", TokenType::Or),
                ("||", TokenType::Or),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("null", TokenType::Null),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("return", TokenType::Return),
                ("print", TokenType::Print),
                ("while", TokenType::While),
                ("for", TokenType::For),
                ("let", TokenType::Let),
                ("this", TokenType::This),
                ("extends", TokenType::Extends),
                ("fn", TokenType::Fn),
                ("class", TokenType::Class),
            ])
        }
    }

    #[allow(dead_code)]
    pub fn debug(&mut self) {
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while ! self.is_at_end() {
            self.start = self.current;

            match self.scan_token() {
                Ok(_) => {}
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line_number: 0,
            }
        );

        if errors.len() > 0 {
            let mut message = "".to_string();
            let _ = errors.iter().map(|x| {
                message.push_str(x);
                message.push_str("\n");
            });
            return Err(message);
        }

        Ok(self.tokens.clone())
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c: char = self.advance();

        return match c {
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
            '!' => {
                let token = if self.char_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token)
            },
            '=' => {
                let token = if self.char_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token)
            },
            '<' => {
                let token = if self.char_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token)
            },
            '>' => {
                let token = if self.char_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token)
            },
            '/' => {
                if self.char_match('/') {
                    return self.single_line_comment();
                }

                if self.char_match('*') {
                    return self.multi_line_comment();
                }

                return self.add_token(TokenType::Slash)
            },
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            },
            '"' => {
                return self.string();
            },
            c => {
                if is_digit(c) {
                    return self.number();
                }

                if is_alpha(c) {
                    return self.identifier();
                }

                return Err(format!("Unrecognizable token at line {}", self.line));
            },
        };
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        return c as char;
    }

    fn add_token(self: &mut Self, token_type: TokenType) -> Result<(), String> {
        return self.add_token_literal(token_type, None);
    }

    fn add_token_literal(
        self: &mut Self,
        token_type: TokenType,
        literal: Option<LiteralValue>,
    ) -> Result<(), String> {

        self.tokens.push(Token {
            token_type,
            lexeme: self.current_text(),
            literal,
            line_number: self.line,
        });

        return Ok(());
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn char_match(self: &mut Self, char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != char {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.current_text().parse::<f64>().unwrap();

        return self.add_token_literal(TokenType::Number, Some(FloatValue(value)));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\n';
        }

        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn current_text(&self) -> String {
        return self.text(self.start, self.current);
    }

    fn text(&self, start: usize, end: usize) -> String {
        return String::from(&self.source[start..end]);
    }

    fn single_line_comment(&mut self) -> Result<(), String> {
        while ! self.char_match('\n') {
            self.advance();
        }

        return Ok(());
    }

    fn multi_line_comment(&mut self) -> Result<(), String> {
        while self.peek() != '*' && self.peek_next() != '/' {
            self.advance();
        }

        // Advance twice to skip the next two tokens.
        // TODO: create a function advance that can accept an integer of "hops"

        self.advance();
        self.advance();

        return Ok(());
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && ! self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Exception::throw( "Unterminated string".to_string(), self.line);
        }

        self.advance();

        let value = self.text(self.start + 1, self.current - 1);
        return self.add_token_literal(TokenType::String, Some(StringValue(value)));
    }

    fn identifier(&mut self) -> Result<(), String> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];
        if self.keywords.contains_key(substring) {
            return self.add_token(self.keywords.get(substring).unwrap().clone())
        }

        return self.add_token(TokenType::Identifier);
    }
}

fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;

    return (uch >= 'a' as u8 && uch <= 'z' as u8)
        || (uch >= 'A' as u8 && uch <= 'Z' as u8)
        || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return (ch as u8) >= ('0' as u8) && (ch as u8) <= ('9' as u8);
}

fn is_alpha_numeric(ch: char) -> bool {
    return is_digit(ch) || is_alpha(ch);
}

#[cfg(test)]
#[path = "test_scanner.rs"]
mod test_scanner;