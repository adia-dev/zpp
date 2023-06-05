use super::keyword::Keyword;

#[derive(Default, Debug, Clone, PartialEq, Eq)] // Example derives
pub enum TokenType {
    #[default]
    ILLEGAL, // Represents an illegal or invalid character
    EOF,              // Represents the end of file marker
    IDENT,            // Represents an identifier
    INT,              // Represents an integer
    ASSIGN,           // Represents the assignment operator (=)
    LESS,             // Represents the assignment operator (<)
    GREATER,          // Represents the assignment operator (>)
    ARITHMETIC,       // Represents arithmetic operators (+, -, *, /, %)
    BITOP,            // Represents bitwise operators (&, |, ~, ^)
    COMMA,            // Represents a comma (,)
    COLON,            // Represents a colon (:)
    SEMICOLON,        // Represents a semicolon (;)
    DQUOTE,           // Represents a semicolon (")
    QUOTE,            // Represents a semicolon (')
    BACKTICK,         // Represents a semicolon (`)
    LPAREN,           // Represents a left parenthesis (()
    RPAREN,           // Represents a right parenthesis ())
    LBRACE,           // Represents a left brace ({)
    RBRACE,           // Represents a right brace (})
    KEYWORD(Keyword), // Represents a keyword (custom type)
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
            TokenType::LESS => "LESS",
            TokenType::GREATER => "LESS",
            TokenType::ARITHMETIC => "ARITHMETIC",
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
            TokenType::KEYWORD(kw) => kw.as_str(),
        }
    }

    /// Converts the TokenType variant to a String.
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
            "if" => Keyword::IF,
            "else" => Keyword::ELSE,
            "do" => Keyword::DO,
            "end" => Keyword::END,
            "true" => Keyword::TRUE,
            "false" => Keyword::FALSE,
            _ => Keyword::UNDEFINED,
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
        assert_eq!(TokenType::ARITHMETIC.as_str(), "ARITHMETIC");
        assert_eq!(TokenType::BITOP.as_str(), "BITOP");
        assert_eq!(TokenType::COMMA.as_str(), "COMMA");
        assert_eq!(TokenType::SEMICOLON.as_str(), "SEMICOLON");
        assert_eq!(TokenType::LPAREN.as_str(), "LPAREN");
        assert_eq!(TokenType::RPAREN.as_str(), "RPAREN");
        assert_eq!(TokenType::LBRACE.as_str(), "LBRACE");
        assert_eq!(TokenType::RBRACE.as_str(), "RBRACE");
        assert_eq!(TokenType::KEYWORD(Keyword::LET).as_str(), "LET");
    }

    #[test]
    fn test_token_type_to_string() {
        assert_eq!(TokenType::ILLEGAL.to_string(), "ILLEGAL");
        assert_eq!(TokenType::EOF.to_string(), "EOF");
        assert_eq!(TokenType::IDENT.to_string(), "IDENT");
        assert_eq!(TokenType::INT.to_string(), "INT");
        assert_eq!(TokenType::ASSIGN.to_string(), "ASSIGN");
        assert_eq!(TokenType::ARITHMETIC.to_string(), "ARITHMETIC");
        assert_eq!(TokenType::BITOP.to_string(), "BITOP");
        assert_eq!(TokenType::COMMA.to_string(), "COMMA");
        assert_eq!(TokenType::SEMICOLON.to_string(), "SEMICOLON");
        assert_eq!(TokenType::LPAREN.to_string(), "LPAREN");
        assert_eq!(TokenType::RPAREN.to_string(), "RPAREN");
        assert_eq!(TokenType::LBRACE.to_string(), "LBRACE");
        assert_eq!(TokenType::RBRACE.to_string(), "RBRACE");
        assert_eq!(TokenType::KEYWORD(Keyword::LET).to_string(), "LET");
    }

    #[test]
    fn test_token_type_dispatch_keyword() {
        assert_eq!(TokenType::dispatch_keyword("let"), Keyword::LET);
        assert_eq!(TokenType::dispatch_keyword("const"), Keyword::CONST);
        assert_eq!(TokenType::dispatch_keyword("return"), Keyword::RETURN);
        assert_eq!(TokenType::dispatch_keyword("for"), Keyword::FOR);
        assert_eq!(TokenType::dispatch_keyword("while"), Keyword::WHILE);
        assert_eq!(TokenType::dispatch_keyword("fn"), Keyword::FUNCTION);
        assert_eq!(TokenType::dispatch_keyword("invalid"), Keyword::UNDEFINED);
    }
}
