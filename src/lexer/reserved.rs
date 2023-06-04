pub enum Keyword {
    LET,
    CONST,
    VAR,
    RETURN,
    FOR,
    WHILE,
    FUNCTION,
    UNDEFINED
}

pub enum Reserved {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    ARITHMETIC,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    KEYWORD(Keyword),
}

impl Reserved {
    pub fn as_str(&self) -> &'static str {
        match self {
            Reserved::ILLEGAL => "ILLEGAL",
            Reserved::EOF => "EOF",
            Reserved::IDENT => "IDENT",
            Reserved::INT => "INT",
            Reserved::ASSIGN => "ASSIGN",
            Reserved::ARITHMETIC => "ARITHMETIC",
            Reserved::COMMA => "COMMA",
            Reserved::SEMICOLON => "SEMICOLON",
            Reserved::LBRACE => "LBRACE",
            Reserved::LPAREN => "LPAREN",
            Reserved::RPAREN => "RPAREN",
            Reserved::RBRACE => "RBRACE",
            Reserved::KEYWORD(kw) => kw.as_str()
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    pub fn dispatch_keyword(tok_str: &str) -> Keyword {
        match tok_str {
            "let" => Keyword::LET,
            "const" => Keyword::CONST,
            "return" => Keyword::RETURN,
            "for" => Keyword::FOR,
            "while" => Keyword::WHILE,
            "fn" => Keyword::FUNCTION,
            _ => Keyword::UNDEFINED,
        }
    }
}

impl Keyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::LET => "LET",
            Keyword::CONST => "CONST",
            Keyword::VAR => "VAR",
            Keyword::RETURN => "RETURN",
            Keyword::FOR => "FOR",
            Keyword::WHILE => "WHILE",
            Keyword::FUNCTION => "FUNCTION",
            _ => "IDENT" // TODO: fix later, this isn't quite right since it should be UNDEFINED or
            // something like that 
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
