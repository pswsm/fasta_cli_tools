use std::fmt::{self, Display};

/// Basic codon value representation
pub type CodonValue = [char; 3];

/// Wrapper for CodonValue
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Codon {
    codon: CodonValue,
}

impl Display for Codon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codon.iter().collect::<String>())
    }
}

impl From<[char; 3]> for Codon {
    fn from(value: [char; 3]) -> Self {
        let value_binding = value;
        if value_binding.clone().iter().count() != 3 {
            panic!("Not three (3) bases")
        }
        Codon {
            codon: value_binding,
        }
    }
}

impl Codon {
    /// Create a new Codon from an array of 3 chars
    pub fn from_chars(value: [char; 3]) -> Self {
        let value_binding = value;
        if value_binding.clone().iter().count() != 3 {
            panic!("Not three (3) bases")
        }
        Codon {
            codon: value_binding,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::protein::codon::domain::codon::Codon;

    #[test]
    fn structs_codon() {
        let codon: Codon = Codon::from_chars(['a', 'u', 'g']);
        assert_eq!(codon.to_string(), "aug".to_string())
    }
}
