
#[derive(Debug, Clone, PartialEq, Eq)] // Example derives
pub enum Keyword {
    LET,       // Represents the "let" keyword
    CONST,     // Represents the "const" keyword
    VAR,       // Represents the "var" keyword
    RETURN,    // Represents the "return" keyword
    FOR,       // Represents the "for" keyword
    WHILE,     // Represents the "while" keyword
    FUNCTION,  // Represents the "function" keyword
    UNDEFINED, // Represents an undefined or unrecognized keyword
}

#[derive(Debug, Clone, PartialEq, Eq)] // Example derives
pub enum Reserved {
    ILLEGAL,          // Represents an illegal or invalid character
    EOF,              // Represents the end of file marker
    IDENT,            // Represents an identifier
    INT,              // Represents an integer
    ASSIGN,           // Represents the assignment operator (=)
    ARITHMETIC,       // Represents arithmetic operators (+, -, *, /, %)
    BIT_OPERATOR,     // Represents bitwise operators (&, |, ~, ^)
    COMMA,            // Represents a comma (,)
    SEMICOLON,        // Represents a semicolon (;)
    LPAREN,           // Represents a left parenthesis (()
    RPAREN,           // Represents a right parenthesis ())
    LBRACE,           // Represents a left brace ({)
    RBRACE,           // Represents a right brace (})
    KEYWORD(Keyword), // Represents a keyword (custom type)
}


impl Reserved {
    /// Returns the string representation of the Reserved variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Reserved::ILLEGAL => "ILLEGAL",
            Reserved::EOF => "EOF",
            Reserved::IDENT => "IDENT",
            Reserved::INT => "INT",
            Reserved::ASSIGN => "ASSIGN",
            Reserved::ARITHMETIC => "ARITHMETIC",
            Reserved::BIT_OPERATOR => "BIT_OPERATOR",
            Reserved::COMMA => "COMMA",
            Reserved::SEMICOLON => "SEMICOLON",
            Reserved::LPAREN => "LPAREN",
            Reserved::RPAREN => "RPAREN",
            Reserved::LBRACE => "LBRACE",
            Reserved::RBRACE => "RBRACE",
            Reserved::KEYWORD(kw) => kw.as_str(),
        }
    }

    /// Converts the Reserved variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Dispatches the corresponding Keyword variant based on the given token string.
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
    /// Returns the string representation of the Keyword variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::LET => "LET",
            Keyword::CONST => "CONST",
            Keyword::VAR => "VAR",
            Keyword::RETURN => "RETURN",
            Keyword::FOR => "FOR",
            Keyword::WHILE => "WHILE",
            Keyword::FUNCTION => "FUNCTION",
            _ => "IDENT",
        }
    }

    /// Converts the Keyword variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserved_as_str() {
        assert_eq!(Reserved::ILLEGAL.as_str(), "ILLEGAL");
        assert_eq!(Reserved::EOF.as_str(), "EOF");
        assert_eq!(Reserved::IDENT.as_str(), "IDENT");
        assert_eq!(Reserved::INT.as_str(), "INT");
        assert_eq!(Reserved::ASSIGN.as_str(), "ASSIGN");
        assert_eq!(Reserved::ARITHMETIC.as_str(), "ARITHMETIC");
        assert_eq!(Reserved::BIT_OPERATOR.as_str(), "BIT_OPERATOR");
        assert_eq!(Reserved::COMMA.as_str(), "COMMA");
        assert_eq!(Reserved::SEMICOLON.as_str(), "SEMICOLON");
        assert_eq!(Reserved::LPAREN.as_str(), "LPAREN");
        assert_eq!(Reserved::RPAREN.as_str(), "RPAREN");
        assert_eq!(Reserved::LBRACE.as_str(), "LBRACE");
        assert_eq!(Reserved::RBRACE.as_str(), "RBRACE");
        assert_eq!(Reserved::KEYWORD(Keyword::LET).as_str(), "LET");
    }

    #[test]
    fn test_reserved_to_string() {
        assert_eq!(Reserved::ILLEGAL.to_string(), "ILLEGAL");
        assert_eq!(Reserved::EOF.to_string(), "EOF");
        assert_eq!(Reserved::IDENT.to_string(), "IDENT");
        assert_eq!(Reserved::INT.to_string(), "INT");
        assert_eq!(Reserved::ASSIGN.to_string(), "ASSIGN");
        assert_eq!(Reserved::ARITHMETIC.to_string(), "ARITHMETIC");
        assert_eq!(Reserved::BIT_OPERATOR.to_string(), "BIT_OPERATOR");
        assert_eq!(Reserved::COMMA.to_string(), "COMMA");
        assert_eq!(Reserved::SEMICOLON.to_string(), "SEMICOLON");
        assert_eq!(Reserved::LPAREN.to_string(), "LPAREN");
        assert_eq!(Reserved::RPAREN.to_string(), "RPAREN");
        assert_eq!(Reserved::LBRACE.to_string(), "LBRACE");
        assert_eq!(Reserved::RBRACE.to_string(), "RBRACE");
        assert_eq!(Reserved::KEYWORD(Keyword::LET).to_string(), "LET");
    }

    #[test]
    fn test_reserved_dispatch_keyword() {
        assert_eq!(Reserved::dispatch_keyword("let"), Keyword::LET);
        assert_eq!(Reserved::dispatch_keyword("const"), Keyword::CONST);
        assert_eq!(Reserved::dispatch_keyword("return"), Keyword::RETURN);
        assert_eq!(Reserved::dispatch_keyword("for"), Keyword::FOR);
        assert_eq!(Reserved::dispatch_keyword("while"), Keyword::WHILE);
        assert_eq!(Reserved::dispatch_keyword("fn"), Keyword::FUNCTION);
        assert_eq!(Reserved::dispatch_keyword("invalid"), Keyword::UNDEFINED);
    }

    #[test]
    fn test_keyword_as_str() {
        assert_eq!(Keyword::LET.as_str(), "LET");
        assert_eq!(Keyword::CONST.as_str(), "CONST");
        assert_eq!(Keyword::VAR.as_str(), "VAR");
        assert_eq!(Keyword::RETURN.as_str(), "RETURN");
        assert_eq!(Keyword::FOR.as_str(), "FOR");
        assert_eq!(Keyword::WHILE.as_str(), "WHILE");
        assert_eq!(Keyword::FUNCTION.as_str(), "FUNCTION");
        assert_eq!(Keyword::UNDEFINED.as_str(), "IDENT");
    }

    #[test]
    fn test_keyword_to_string() {
        assert_eq!(Keyword::LET.to_string(), "LET");
        assert_eq!(Keyword::CONST.to_string(), "CONST");
        assert_eq!(Keyword::VAR.to_string(), "VAR");
        assert_eq!(Keyword::RETURN.to_string(), "RETURN");
        assert_eq!(Keyword::FOR.to_string(), "FOR");
        assert_eq!(Keyword::WHILE.to_string(), "WHILE");
        assert_eq!(Keyword::FUNCTION.to_string(), "FUNCTION");
        assert_eq!(Keyword::UNDEFINED.to_string(), "IDENT");
    }
}
