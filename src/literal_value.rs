#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    NumberValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}