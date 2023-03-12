use crate::literal_value::LiteralValue;
use crate::token::Token;
use crate::token_type::TokenType;

pub enum ExpressionLiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null,
}

impl ExpressionLiteralValue {
    pub fn to_string(&self) -> String {
        return match self {
            ExpressionLiteralValue::Number(n) => n.to_string(),
            ExpressionLiteralValue::StringValue(s) => s.clone(),
            ExpressionLiteralValue::True => String::from("true"),
            ExpressionLiteralValue::False => String::from("false"),
            ExpressionLiteralValue::Null => String::from("null"),
        }
    }

    pub fn from_token(token: Token) -> ExpressionLiteralValue {
        return match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::String => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Null => Self::Null,
            _ => panic!("Could not create ExpressionLiteralValue from {:?}", token)
        }
    }
}

fn unwrap_as_string(literal: Option<LiteralValue>) -> String {
    match literal.unwrap() {
        LiteralValue::StringValue(s) | LiteralValue::IdentifierValue(s) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

fn unwrap_as_f32(literal: Option<LiteralValue>) -> f32 {
    match literal.unwrap() {
        LiteralValue::FloatValue(x) => x as f32,
        LiteralValue::IntegerValue(x) => x as f32,
        _ => panic!("Could not unwrap as f32"),
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        value: ExpressionLiteralValue
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        return match self {
            Expr::Binary { left, operator, right } => {
                format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string())
            },
            Expr::Grouping { expression } => {
                format!("(group {})", expression.to_string())
            },
            Expr::Literal { value } => {
                format!("{}", value.to_string())
            },
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, (*right).to_string())
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
    use crate::token::Token;
    use crate::token_type::TokenType;
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0
        };
        let number = Box::from(Literal {
            value: ExpressionLiteralValue::Number(123.0)
        });
        let multiplication = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0
        };
        let group = Box::from(Grouping {
            expression: Box::from(Literal {
                value: ExpressionLiteralValue::Number(420.69)
            }),
        });

        let ast = Binary {
            left: Box::from(Unary {
                operator: minus,
                right: number,
            }),
            operator: multiplication,
            right: group,
        };

        assert_eq!(ast.to_string(), "(* (- 123) (group 420.69))".to_string());
    }
}