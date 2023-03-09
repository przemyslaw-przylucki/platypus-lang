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
                    while self.peek() == '\n' || !self.is_at_end() {
                        self.advance();
                    }

                    return Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            },
            '"' => {
                self.string();
                Ok(())
            }
            c => {
                if is_digit(c) {
                    self.number();
                    return Ok(())
                } else {}

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
        self.add_token_literal(TokenType::Number, Some(FloatValue(value)));

        return Ok(());
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
        self.add_token_literal(TokenType::String, Some(StringValue(value)));

        return Ok(());
    }
}

fn is_digit(ch: char) -> bool {
    return (ch as u8) >= ('0' as u8) && (ch as u8) <= ('9' as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_one_char_tokens() {
        let source = "{(( ))}";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 7);
        println!("{:?}", scanner.tokens);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[2].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[3].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[4].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[5].token_type, TokenType::RightBrace);
        assert_eq!(scanner.tokens[6].token_type, TokenType::Eof);
    }

    #[test]
    fn handles_operators() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Bang);
        assert_eq!(scanner.tokens[1].token_type, TokenType::BangEqual);
        assert_eq!(scanner.tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, TokenType::GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn handles_number_literals() {
        let source = "420 69 420.69";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 4);


        assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
        // assert_eq!(scanner.tokens[0].literal, LiteralValue::FloatValue);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Number);
        // assert_eq!(scanner.tokens[1].literal, LiteralValue::FloatValue);
        assert_eq!(scanner.tokens[2].token_type, TokenType::Number);
        // assert_eq!(scanner.tokens[2].literal, LiteralValue::FloatValue);

        assert_eq!(scanner.tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn handles_string_literals() {
        let source = r#""platypus""#;
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);

        assert_eq!(scanner.tokens[0].token_type, TokenType::String);

        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "platypus"),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn handles_string_literals_unterminated() {
        let source = r#""platypus"#;
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        match result {
            Err(_) => (),
            _ => panic!("Test didn't fail but it should"),
        }
    }
}