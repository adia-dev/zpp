#[derive(Debug, Clone, PartialEq, Eq)] // Example derives
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
    DO,        // Represents the "DO" keyword
    END,       // Represents the "END" keyword
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
}
