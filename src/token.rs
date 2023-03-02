use crate::literal_value::LiteralValue;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<LiteralValue>,
    pub(crate) line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        return format!("{} {} {:?}", self.token_type, self.lexeme, self.literal).to_string();
    }
}