use crate::expr::{Expr, ExpressionLiteralValue};
use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        return self.expression();
    }

    fn expression(&mut self) -> Result<Expr, String> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_token_type(vec!(TokenType::BangEqual, TokenType::EqualEqual)) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            };
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token_type(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }

        return Ok(expr);
    }

    fn match_token_type(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn previous(&self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().clone();
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token;
    }

    fn peek(&self) -> Token {
        return self.tokens.get(self.current).unwrap().clone();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn advance(&mut self) -> Token {
        if ! self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Unary {
                operator,
                right: Box::from(right),
            })
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();

        let result;

        match token.token_type {
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume_token(TokenType::RightParen, "Expected ')'");

                result = Grouping {
                    expression: Box::from(expr),
                }
            },
            TokenType::False | TokenType::True | TokenType::Null | TokenType::Number | TokenType::String => {
                self.advance();
                result = Literal {
                    value: ExpressionLiteralValue::from_token(token),
                }
            }
            _ => return Err("Expected expression".to_string()),
        }

        return Ok(result);
    }

    fn consume_token(&mut self, token_type: TokenType, message: &str) -> Result<(), String> {
        let token = self.peek();

        if token.token_type != token_type {
            return Err(message.to_string());
        }

        self.advance();

        return Ok(())
    }

    fn synchronize(&mut self) {
        self.advance();

        while ! self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fn | TokenType::Let | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::literal_value::LiteralValue::IntegerValue;
    use crate::scanner::Scanner;
    use super::*;

    #[test]
    fn test_addition() {
            let one = Token {
                token_type: TokenType::Number,
                lexeme: "1".to_string(),
                literal: Some(IntegerValue(1)),
                line_number: 0,
            };
            let plus = Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line_number: 0,
            };
            let two = Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                literal: Some(IntegerValue(2)),
                line_number: 0,
            };
            let semicolon = Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                literal: Some(IntegerValue(2)),
                line_number: 0,
            };

        let tokens = vec![one,plus,two,semicolon];
        let mut parser = Parser::new(tokens);

        let parsed_expression = parser.parse().unwrap();
        let string_expr = parsed_expression.to_string();

        parsed_expression.print();
        assert_eq!(string_expr, "(+ 1 2)")
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner.scan_tokens().unwrap());
        let string_expression = parser.parse().unwrap().to_string();

        assert_eq!(string_expression, "(== (+ 1 2) (+ 5 7))")
    }
}
