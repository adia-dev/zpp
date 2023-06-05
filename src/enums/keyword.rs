#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    LET,       // Represents the "let" keyword
    CONST,     // Represents the "const" keyword
    VAR,       // Represents the "var" keyword
    RETURN,    // Represents the "return" keyword
    FOR,       // Represents the "for" keyword
    WHILE,     // Represents the "while" keyword
    FUNCTION,  // Represents the "function" keyword
    IF,        // Represents the "if" keyword
    ELSE,      // Represents the "else" keyword
    DO,        // Represents the "do" keyword
    END,       // Represents the "end" keyword
    TRUE,      // Represents the "true" keyword
    FALSE,     // Represents the "false" keyword
    UNDEFINED, // Represents an undefined or unrecognized keyword
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
            Keyword::IF => "IF",
            Keyword::ELSE => "ELSE",
            Keyword::DO => "DO",
            Keyword::END => "END",
            Keyword::TRUE => "TRUE",
            Keyword::FALSE => "FALSE",
            _ => "IDENT",
        }
    }

    /// Converts the Keyword variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Fetches the corresponding Keyword variant based on the given string.
    pub fn from_str(tok_str: &str) -> Option<Keyword> {
        match tok_str {
            "let" => Some(Keyword::LET),
            "const" => Some(Keyword::CONST),
            "var" => Some(Keyword::VAR),
            "return" => Some(Keyword::RETURN),
            "for" => Some(Keyword::FOR),
            "while" => Some(Keyword::WHILE),
            "fn" => Some(Keyword::FUNCTION),
            "if" => Some(Keyword::IF),
            "else" => Some(Keyword::ELSE),
            "do" => Some(Keyword::DO),
            "end" => Some(Keyword::END),
            "true" => Some(Keyword::TRUE),
            "false" => Some(Keyword::FALSE),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_keyword_from_str() {
        assert_eq!(Keyword::from_str("let"), Some(Keyword::LET));
        assert_eq!(Keyword::from_str("const"), Some(Keyword::CONST));
        assert_eq!(Keyword::from_str("var"), Some(Keyword::VAR));
        assert_eq!(Keyword::from_str("return"), Some(Keyword::RETURN));
        assert_eq!(Keyword::from_str("for"), Some(Keyword::FOR));
        assert_eq!(Keyword::from_str("while"), Some(Keyword::WHILE));
        assert_eq!(Keyword::from_str("fn"), Some(Keyword::FUNCTION));
        assert_eq!(Keyword::from_str("if"), Some(Keyword::IF));
        assert_eq!(Keyword::from_str("else"), Some(Keyword::ELSE));
        assert_eq!(Keyword::from_str("do"), Some(Keyword::DO));
        assert_eq!(Keyword::from_str("end"), Some(Keyword::END));
        assert_eq!(Keyword::from_str("true"), Some(Keyword::TRUE));
        assert_eq!(Keyword::from_str("false"), Some(Keyword::FALSE));
        assert_eq!(Keyword::from_str("undefined"), None);
    }
}
