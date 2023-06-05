use super::{arithmetic::Arithmetic, cmp::Cmp, keyword::Keyword, bitop::Bitop, logicop::LogicOp};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    #[default]
    ILLEGAL,    // Represents an illegal or invalid character
    EOF,        // Represents the end of file marker
    IDENT,      // Represents an identifier
    INT,        // Represents an integer
    ASSIGN,     // Represents the assignment operator (=)
    COMMA,      // Represents a comma (,)
    COLON,      // Represents a colon (:)
    SEMICOLON,  // Represents a semicolon (;)
    DQUOTE,     // Represents a double quote (")
    QUOTE,      // Represents a single quote (')
    BACKTICK,   // Represents a backtick (`)
    LPAREN,     // Represents a left parenthesis (()
    RPAREN,     // Represents a right parenthesis ())
    LBRACE,     // Represents a left brace ({)
    RBRACE,     // Represents a right brace (})
    CMP(Cmp),   // Represents comparison operators
    ARITHMETIC(Arithmetic), // Represents arithmetic operators
    BITOP(Bitop), // Represents bitwise operators
    LOGICOP(LogicOp), // Represents logical operators
    KEYWORD(Keyword),   // Represents a keyword (custom type)
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
            TokenType::COMMA => "COMMA",
            TokenType::COLON => "COLON",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::DQUOTE => "DQUOTE",
            TokenType::QUOTE => "QUOTE",
            TokenType::BACKTICK => "BACKTICK",
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::LBRACE => "LBRACE",
            TokenType::RBRACE => "RBRACE",
            TokenType::CMP(c) => c.as_str(),
            TokenType::ARITHMETIC(a) => a.as_str(),
            TokenType::BITOP(b) => b.as_str(),
            TokenType::LOGICOP(l) => l.as_str(),
            TokenType::KEYWORD(kw) => kw.as_str(),
        }
    }

    /// Converts the TokenType variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Creates a TokenType variant from a string representation.
    pub fn from_str(value: &str) -> Option<TokenType> {
        match value {
            "ILLEGAL" => Some(TokenType::ILLEGAL),
            "EOF" => Some(TokenType::EOF),
            "IDENT" => Some(TokenType::IDENT),
            "INT" => Some(TokenType::INT),
            "ASSIGN" => Some(TokenType::ASSIGN),
            "COMMA" => Some(TokenType::COMMA),
            "COLON" => Some(TokenType::COLON),
            "SEMICOLON" => Some(TokenType::SEMICOLON),
            "DQUOTE" => Some(TokenType::DQUOTE),
            "QUOTE" => Some(TokenType::QUOTE),
            "BACKTICK" => Some(TokenType::BACKTICK),
            "LPAREN" => Some(TokenType::LPAREN),
            "RPAREN" => Some(TokenType::RPAREN),
            "LBRACE" => Some(TokenType::LBRACE),
            "RBRACE" => Some(TokenType::RBRACE),
            _ => {
                if let Some(cmp) = Cmp::from_str(value) {
                    Some(TokenType::CMP(cmp))
                } else if let Some(arithmetic) = Arithmetic::from_str(value) {
                    Some(TokenType::ARITHMETIC(arithmetic))
                } else if let Some(bitop) = Bitop::from_str(value) {
                    Some(TokenType::BITOP(bitop))
                } else if let Some(logicop) = LogicOp::from_str(value) {
                    Some(TokenType::LOGICOP(logicop))
                } else if let Some(keyword) = Keyword::from_str(value) {
                    Some(TokenType::KEYWORD(keyword))
                } else {
                    None
                }
            }
        }
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
        assert_eq!(TokenType::COMMA.as_str(), "COMMA");
        assert_eq!(TokenType::COLON.as_str(), "COLON");
        assert_eq!(TokenType::SEMICOLON.as_str(), "SEMICOLON");
        assert_eq!(TokenType::DQUOTE.as_str(), "DQUOTE");
        assert_eq!(TokenType::QUOTE.as_str(), "QUOTE");
        assert_eq!(TokenType::BACKTICK.as_str(), "BACKTICK");
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
        assert_eq!(TokenType::COMMA.to_string(), "COMMA");
        assert_eq!(TokenType::COLON.to_string(), "COLON");
        assert_eq!(TokenType::SEMICOLON.to_string(), "SEMICOLON");
        assert_eq!(TokenType::DQUOTE.to_string(), "DQUOTE");
        assert_eq!(TokenType::QUOTE.to_string(), "QUOTE");
        assert_eq!(TokenType::BACKTICK.to_string(), "BACKTICK");
        assert_eq!(TokenType::LPAREN.to_string(), "LPAREN");
        assert_eq!(TokenType::RPAREN.to_string(), "RPAREN");
        assert_eq!(TokenType::LBRACE.to_string(), "LBRACE");
        assert_eq!(TokenType::RBRACE.to_string(), "RBRACE");
    }

    #[test]
    fn test_token_type_from_str() {
        assert_eq!(TokenType::from_str("ILLEGAL"), Some(TokenType::ILLEGAL));
        assert_eq!(TokenType::from_str("EOF"), Some(TokenType::EOF));
        assert_eq!(TokenType::from_str("IDENT"), Some(TokenType::IDENT));
        assert_eq!(TokenType::from_str("INT"), Some(TokenType::INT));
        assert_eq!(TokenType::from_str("ASSIGN"), Some(TokenType::ASSIGN));
        assert_eq!(TokenType::from_str("COMMA"), Some(TokenType::COMMA));
        assert_eq!(TokenType::from_str("COLON"), Some(TokenType::COLON));
        assert_eq!(TokenType::from_str("SEMICOLON"), Some(TokenType::SEMICOLON));
        assert_eq!(TokenType::from_str("DQUOTE"), Some(TokenType::DQUOTE));
        assert_eq!(TokenType::from_str("QUOTE"), Some(TokenType::QUOTE));
        assert_eq!(TokenType::from_str("BACKTICK"), Some(TokenType::BACKTICK));
        assert_eq!(TokenType::from_str("LPAREN"), Some(TokenType::LPAREN));
        assert_eq!(TokenType::from_str("RPAREN"), Some(TokenType::RPAREN));
        assert_eq!(TokenType::from_str("LBRACE"), Some(TokenType::LBRACE));
        assert_eq!(TokenType::from_str("RBRACE"), Some(TokenType::RBRACE));
    }
}
