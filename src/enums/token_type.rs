use super::{arithmetic::Arithmetic, cmp::Cmp, keyword::Keyword};

#[derive(Default, Debug, Clone, PartialEq, Eq)] // Example derives
pub enum TokenType {
    #[default]
    ILLEGAL, // Represents an illegal or invalid character
    EOF,                    // Represents the end of file marker
    IDENT,                  // Represents an identifier
    INT,                    // Represents an integer
    ASSIGN,                 // Represents the assignment operator (=)
    BITOP,                  // Represents bitwise operators (&, |, ~, ^)
    COMMA,                  // Represents a comma (,)
    COLON,                  // Represents a colon (:)
    SEMICOLON,              // Represents a semicolon (;)
    DQUOTE,                 // Represents a semicolon (")
    QUOTE,                  // Represents a semicolon (')
    BACKTICK,               // Represents a semicolon (`)
    LPAREN,                 // Represents a left parenthesis (()
    RPAREN,                 // Represents a right parenthesis ())
    LBRACE,                 // Represents a left brace ({)
    RBRACE,                 // Represents a right brace (})
    CMP(Cmp),               // Represents comparison operators
    ARITHMETIC(Arithmetic), // Represents arithmetic operators
    KEYWORD(Keyword),       // Represents a keyword (custom type)
}

impl TokenType {
    /// Returns the string representation of the TokenType variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::EOF => "EOF",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::ASSIGN => "ASSIGN",
            TokenType::BITOP => "BITOP",
            TokenType::COMMA => "COMMA",
            TokenType::COLON => "COLON",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::QUOTE => "QUOTE",
            TokenType::DQUOTE => "DQUOTE",
            TokenType::BACKTICK => "BACKTICK",
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::LBRACE => "LBRACE",
            TokenType::RBRACE => "RBRACE",
            TokenType::CMP(c) => c.as_str(),
            TokenType::ARITHMETIC(a) => a.as_str(),
            TokenType::KEYWORD(kw) => kw.as_str(),
        }
    }

    /// Converts the TokenType variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_as_str() {
        assert_eq!(TokenType::ILLEGAL.as_str(), "ILLEGAL");
        assert_eq!(TokenType::EOF.as_str(), "EOF");
        assert_eq!(TokenType::IDENT.as_str(), "IDENT");
        assert_eq!(TokenType::INT.as_str(), "INT");
        assert_eq!(TokenType::ASSIGN.as_str(), "ASSIGN");
        assert_eq!(TokenType::BITOP.as_str(), "BITOP");
        assert_eq!(TokenType::COMMA.as_str(), "COMMA");
        assert_eq!(TokenType::SEMICOLON.as_str(), "SEMICOLON");
        assert_eq!(TokenType::LPAREN.as_str(), "LPAREN");
        assert_eq!(TokenType::RPAREN.as_str(), "RPAREN");
        assert_eq!(TokenType::LBRACE.as_str(), "LBRACE");
        assert_eq!(TokenType::RBRACE.as_str(), "RBRACE");
    }

    #[test]
    fn test_token_type_to_string() {
        assert_eq!(TokenType::ILLEGAL.to_string(), "ILLEGAL");
        assert_eq!(TokenType::EOF.to_string(), "EOF");
        assert_eq!(TokenType::IDENT.to_string(), "IDENT");
        assert_eq!(TokenType::INT.to_string(), "INT");
        assert_eq!(TokenType::ASSIGN.to_string(), "ASSIGN");
        assert_eq!(TokenType::BITOP.to_string(), "BITOP");
        assert_eq!(TokenType::COMMA.to_string(), "COMMA");
        assert_eq!(TokenType::SEMICOLON.to_string(), "SEMICOLON");
        assert_eq!(TokenType::LPAREN.to_string(), "LPAREN");
        assert_eq!(TokenType::RPAREN.to_string(), "RPAREN");
        assert_eq!(TokenType::LBRACE.to_string(), "LBRACE");
        assert_eq!(TokenType::RBRACE.to_string(), "RBRACE");
    }
}
