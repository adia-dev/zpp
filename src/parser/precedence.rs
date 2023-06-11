#[derive(Debug, Clone)]
pub enum Precedence {
    Lowest = 1,
    EQ,
    LGT,
    Sum,
    Product,
    Prefix,
    Call,
}

impl ToString for Precedence {
    fn to_string(&self) -> String {
        match self {
            Precedence::Lowest => "Lowest".to_owned(),
            Precedence::EQ => "EQ".to_owned(),
            Precedence::LGT => "LGT".to_owned(),
            Precedence::Sum => "sum".to_owned(),
            Precedence::Product => "Product".to_owned(),
            Precedence::Prefix => "Prefix".to_owned(),
            Precedence::Call => "Call".to_owned(),
        }
    }
}
