#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmp {
    GT,     // Greater than operator (>)
    LT,     // Less than operator (<)
    GE,     // Greater than or equal to operator (>=)
    LE,     // Less than or equal to operator (<=)
    EQUAL,  // Equality operator (==)
    NEQUAL, // Not equal to operator (!=)
}

impl Cmp {
    /// Returns the string representation of the Cmp variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Cmp::GT => "GT",
            Cmp::LT => "LT",
            Cmp::GE => "GE",
            Cmp::LE => "LE",
            Cmp::EQUAL => "EQUAL",
            Cmp::NEQUAL => "NEQUAL",
        }
    }

    /// Converts the Cmp variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Creates a Cmp variant from a string representation.
    pub fn from_str(value: &str) -> Option<Cmp> {
        match value {
            "GT" | ">" => Some(Cmp::GT),
            "LT" | "<" => Some(Cmp::LT),
            "GE" | ">=" => Some(Cmp::GE),
            "LE" | "<=" => Some(Cmp::LE),
            "EQUAL" | "==" => Some(Cmp::EQUAL),
            "NEQUAL" | "!=" => Some(Cmp::NEQUAL),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(Cmp::GT.as_str(), "GT");
        assert_eq!(Cmp::LT.as_str(), "LT");
        assert_eq!(Cmp::GE.as_str(), "GE");
        assert_eq!(Cmp::LE.as_str(), "LE");
        assert_eq!(Cmp::EQUAL.as_str(), "EQUAL");
        assert_eq!(Cmp::NEQUAL.as_str(), "NEQUAL");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Cmp::GT.to_string(), "GT");
        assert_eq!(Cmp::LT.to_string(), "LT");
        assert_eq!(Cmp::GE.to_string(), "GE");
        assert_eq!(Cmp::LE.to_string(), "LE");
        assert_eq!(Cmp::EQUAL.to_string(), "EQUAL");
        assert_eq!(Cmp::NEQUAL.to_string(), "NEQUAL");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Cmp::from_str(">"), Some(Cmp::GT));
        assert_eq!(Cmp::from_str("<"), Some(Cmp::LT));
        assert_eq!(Cmp::from_str(">="), Some(Cmp::GE));
        assert_eq!(Cmp::from_str("<="), Some(Cmp::LE));
        assert_eq!(Cmp::from_str("=="), Some(Cmp::EQUAL));
        assert_eq!(Cmp::from_str("!="), Some(Cmp::NEQUAL));

        assert_eq!(Cmp::from_str("GT"), Some(Cmp::GT));
        assert_eq!(Cmp::from_str("LT"), Some(Cmp::LT));
        assert_eq!(Cmp::from_str("GE"), Some(Cmp::GE));
        assert_eq!(Cmp::from_str("LE"), Some(Cmp::LE));
        assert_eq!(Cmp::from_str("EQUAL"), Some(Cmp::EQUAL));
        assert_eq!(Cmp::from_str("NEQUAL"), Some(Cmp::NEQUAL));

        assert_eq!(Cmp::from_str("INVALID"), None);
    }
}
