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

    pub fn dispatch_keyword(tok_str: &str) -> Reserved {
        match tok_str {
             "fn" => Reserved::FUNCTION,
             "let" => Reserved::LET,
            _ => Reserved::IDENT
        }
    }
}
