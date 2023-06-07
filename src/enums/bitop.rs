#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bitop {
    AND, // Bitwise AND operator (&)
    OR,  // Bitwise OR operator (|)
    XOR, // Bitwise XOR operator (^)
    NOT, // Bitwise NOT operator (~)
    SHL, // Left shift operator (<<)
    SHR, // Right shift operator (>>)
}

impl Bitop {
    /// Returns the string representation of the Bitop variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Bitop::AND => "AND",
            Bitop::OR => "OR",
            Bitop::XOR => "XOR",
            Bitop::NOT => "NOT",
            Bitop::SHL => "SHL",
            Bitop::SHR => "SHR",
        }
    }

    /// Converts the Bitop variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Creates a Bitop variant from a string representation.
    pub fn from_str(value: &str) -> Option<Bitop> {
        match value {
            "AND" | "&" => Some(Bitop::AND),
            "OR" | "|" => Some(Bitop::OR),
            "XOR" | "^" => Some(Bitop::XOR),
            "NOT" | "~" => Some(Bitop::NOT),
            "SHL" | "<<" => Some(Bitop::SHL),
            "SHR" | ">>" => Some(Bitop::SHR),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(Bitop::AND.as_str(), "AND");
        assert_eq!(Bitop::OR.as_str(), "OR");
        assert_eq!(Bitop::XOR.as_str(), "XOR");
        assert_eq!(Bitop::NOT.as_str(), "NOT");
        assert_eq!(Bitop::SHL.as_str(), "SHL");
        assert_eq!(Bitop::SHR.as_str(), "SHR");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Bitop::AND.to_string(), "AND");
        assert_eq!(Bitop::OR.to_string(), "OR");
        assert_eq!(Bitop::XOR.to_string(), "XOR");
        assert_eq!(Bitop::NOT.to_string(), "NOT");
        assert_eq!(Bitop::SHL.to_string(), "SHL");
        assert_eq!(Bitop::SHR.to_string(), "SHR");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Bitop::from_str("&"), Some(Bitop::AND));
        assert_eq!(Bitop::from_str("|"), Some(Bitop::OR));
        assert_eq!(Bitop::from_str("^"), Some(Bitop::XOR));
        assert_eq!(Bitop::from_str("~"), Some(Bitop::NOT));
        assert_eq!(Bitop::from_str("<<"), Some(Bitop::SHL));
        assert_eq!(Bitop::from_str(">>"), Some(Bitop::SHR));

        assert_eq!(Bitop::from_str("AND"), Some(Bitop::AND));
        assert_eq!(Bitop::from_str("OR"), Some(Bitop::OR));
        assert_eq!(Bitop::from_str("XOR"), Some(Bitop::XOR));
        assert_eq!(Bitop::from_str("NOT"), Some(Bitop::NOT));
        assert_eq!(Bitop::from_str("SHL"), Some(Bitop::SHL));
        assert_eq!(Bitop::from_str("SHR"), Some(Bitop::SHR));

        assert_eq!(Bitop::from_str("INVALID"), None);
    }
}
