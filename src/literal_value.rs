#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    IntegerValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}