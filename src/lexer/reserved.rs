pub enum Reserved {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

impl Reserved {
    pub fn as_str(&self) -> &'static str {
        match self {
            Reserved::ILLEGAL => "ILLEGAL",
            Reserved::EOF => "EOF",
            Reserved::IDENT => "IDENT",
            Reserved::INT => "INT",
            Reserved::ASSIGN => "ASSIGN",
            Reserved::PLUS => "PLUS",
            Reserved::COMMA => "COMMA",
            Reserved::SEMICOLON => "SEMICOLON",
            Reserved::LBRACE => "LBRACE",
            Reserved::LPAREN => "LPAREN",
            Reserved::RPAREN => "RPAREN",
            Reserved::RBRACE => "RBRACE",
            Reserved::FUNCTION => "FUNCTION",
            Reserved::LET => "LET",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Reserved::ILLEGAL => "ILLEGAL".to_string(),
            Reserved::EOF => "EOF".to_string(),
            Reserved::IDENT => "IDENT".to_string(),
            Reserved::INT => "INT".to_string(),
            Reserved::ASSIGN => "ASSIGN".to_string(),
            Reserved::PLUS => "PLUS".to_string(),
            Reserved::COMMA => "COMMA".to_string(),
            Reserved::SEMICOLON => "SEMICOLON".to_string(),
            Reserved::LBRACE => "LBRACE".to_string(),
            Reserved::LPAREN => "LPAREN".to_string(),
            Reserved::RPAREN => "RPAREN".to_string(),
            Reserved::RBRACE => "RBRACE".to_string(),
            Reserved::FUNCTION => "FUNCTION".to_string(),
            Reserved::LET => "LET".to_string(),
        }
    }

    pub fn dispatch_keyword(tok_str: &str) -> Reserved {
        match tok_str {
             "fn" => Reserved::FUNCTION,
             "let" => Reserved::LET,
            _ => Reserved::IDENT
        }
    }
}
