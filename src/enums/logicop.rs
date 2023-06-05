#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicOp {
    AND,    // Logical AND operator (&&)
    OR,     // Logical OR operator (||)
    NOT,    // Logical NOT operator (!)
}

impl LogicOp {
    /// Returns the string representation of the LogicOp variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogicOp::AND => "AND",
            LogicOp::OR => "OR",
            LogicOp::NOT => "NOT",
        }
    }

    /// Converts the LogicOp variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Creates a LogicOp variant from a string representation.
    pub fn from_str(value: &str) -> Option<LogicOp> {
        match value {
            "AND" | "&&" => Some(LogicOp::AND),
            "OR" | "||" => Some(LogicOp::OR),
            "NOT" | "!" => Some(LogicOp::NOT),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(LogicOp::AND.as_str(), "AND");
        assert_eq!(LogicOp::OR.as_str(), "OR");
        assert_eq!(LogicOp::NOT.as_str(), "NOT");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(LogicOp::AND.to_string(), "AND");
        assert_eq!(LogicOp::OR.to_string(), "OR");
        assert_eq!(LogicOp::NOT.to_string(), "NOT");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(LogicOp::from_str("&&"), Some(LogicOp::AND));
        assert_eq!(LogicOp::from_str("||"), Some(LogicOp::OR));
        assert_eq!(LogicOp::from_str("!"), Some(LogicOp::NOT));

        assert_eq!(LogicOp::from_str("AND"), Some(LogicOp::AND));
        assert_eq!(LogicOp::from_str("OR"), Some(LogicOp::OR));
        assert_eq!(LogicOp::from_str("NOT"), Some(LogicOp::NOT));

        assert_eq!(LogicOp::from_str("INVALID"), None);
    }
}
