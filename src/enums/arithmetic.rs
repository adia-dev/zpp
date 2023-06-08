#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Arithmetic {
    PLUS,  // Addition operator (+)
    MINUS, // Subtraction operator (-)
    MUL,   // Multiplication operator (*)
    DIV,   // Division operator (/)
    FDIV,  // Floor division operator (//)
    MOD,   // Modulo operator (%)
    POW,   // Exponentiation operator (**)
    INC,   // Increment operator (++)
    DEC,   // Decrement operator (--)
}

impl Arithmetic {
    /// Returns the string representation of the Arithmetic variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Arithmetic::PLUS => "PLUS",
            Arithmetic::MINUS => "MINUS",
            Arithmetic::MUL => "MUL",
            Arithmetic::DIV => "DIV",
            Arithmetic::FDIV => "FDIV",
            Arithmetic::MOD => "MOD",
            Arithmetic::POW => "POW",
            Arithmetic::INC => "INC",
            Arithmetic::DEC => "DEC",
        }
    }

    /// Converts the Arithmetic variant to a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Creates an Arithmetic variant from a string representation.
    pub fn from_str(value: &str) -> Option<Arithmetic> {
        match value {
            "PLUS" | "+" => Some(Arithmetic::PLUS),
            "MINUS" | "-" => Some(Arithmetic::MINUS),
            "MUL" | "*" => Some(Arithmetic::MUL),
            "DIV" | "/" => Some(Arithmetic::DIV),
            "FDIV" | "//" => Some(Arithmetic::FDIV),
            "MOD" | "%" => Some(Arithmetic::MOD),
            "POW" | "**" => Some(Arithmetic::POW),
            "INC" | "++" => Some(Arithmetic::INC),
            "DEC" | "--" => Some(Arithmetic::DEC),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(Arithmetic::PLUS.as_str(), "PLUS");
        assert_eq!(Arithmetic::MINUS.as_str(), "MINUS");
        assert_eq!(Arithmetic::MUL.as_str(), "MUL");
        assert_eq!(Arithmetic::DIV.as_str(), "DIV");
        assert_eq!(Arithmetic::FDIV.as_str(), "FDIV");
        assert_eq!(Arithmetic::MOD.as_str(), "MOD");
        assert_eq!(Arithmetic::POW.as_str(), "POW");
        assert_eq!(Arithmetic::INC.as_str(), "INC");
        assert_eq!(Arithmetic::DEC.as_str(), "DEC");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Arithmetic::PLUS.to_string(), "PLUS");
        assert_eq!(Arithmetic::MINUS.to_string(), "MINUS");
        assert_eq!(Arithmetic::MUL.to_string(), "MUL");
        assert_eq!(Arithmetic::DIV.to_string(), "DIV");
        assert_eq!(Arithmetic::FDIV.to_string(), "FDIV");
        assert_eq!(Arithmetic::MOD.to_string(), "MOD");
        assert_eq!(Arithmetic::POW.to_string(), "POW");
        assert_eq!(Arithmetic::INC.to_string(), "INC");
        assert_eq!(Arithmetic::DEC.to_string(), "DEC");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Arithmetic::from_str("+"), Some(Arithmetic::PLUS));
        assert_eq!(Arithmetic::from_str("-"), Some(Arithmetic::MINUS));
        assert_eq!(Arithmetic::from_str("*"), Some(Arithmetic::MUL));
        assert_eq!(Arithmetic::from_str("/"), Some(Arithmetic::DIV));
        assert_eq!(Arithmetic::from_str("//"), Some(Arithmetic::FDIV));
        assert_eq!(Arithmetic::from_str("%"), Some(Arithmetic::MOD));
        assert_eq!(Arithmetic::from_str("**"), Some(Arithmetic::POW));
        assert_eq!(Arithmetic::from_str("++"), Some(Arithmetic::INC));
        assert_eq!(Arithmetic::from_str("--"), Some(Arithmetic::DEC));

        assert_eq!(Arithmetic::from_str("PLUS"), Some(Arithmetic::PLUS));
        assert_eq!(Arithmetic::from_str("MINUS"), Some(Arithmetic::MINUS));
        assert_eq!(Arithmetic::from_str("MUL"), Some(Arithmetic::MUL));
        assert_eq!(Arithmetic::from_str("DIV"), Some(Arithmetic::DIV));
        assert_eq!(Arithmetic::from_str("FDIV"), Some(Arithmetic::FDIV));
        assert_eq!(Arithmetic::from_str("MOD"), Some(Arithmetic::MOD));
        assert_eq!(Arithmetic::from_str("POW"), Some(Arithmetic::POW));
        assert_eq!(Arithmetic::from_str("INC"), Some(Arithmetic::INC));
        assert_eq!(Arithmetic::from_str("DEC"), Some(Arithmetic::DEC));

        assert_eq!(Arithmetic::from_str("INVALID"), None);
    }
}
