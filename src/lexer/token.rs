
pub type TokenType = String;

#[derive(Default, Debug)]
pub struct Token {
    // t stands for type. which is a reserved word in rust :/
    pub t: TokenType,
    pub value: String,
    pub line: Option<u32>,
    pub filename: Option<String>,
}

impl Token {
    pub fn new(t: &str, value: &str) -> Token {
        Token {
            t: t.to_string(),
            value: value.to_string(),
            line: None,
            filename: None,
        }
    }
}
