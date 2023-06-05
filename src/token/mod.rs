use crate::enums::token_type::TokenType;

#[derive(Default, Debug)]
pub struct Token {
    // t stands for type. which is a token_type word in rust :/
    pub t: TokenType,
    pub value: String,
    pub line: Option<usize>,
    pub position: Option<usize>,
    pub filename: Option<String>,
}

impl Token {
    pub fn new(
        t: TokenType,
        value: &str,
        line: Option<usize>,
        position: Option<usize>,
        filename: Option<String>,
    ) -> Token {
        Token {
            t,
            value: value.to_string(),
            line,
            position,
            filename,
        }
    }
}
