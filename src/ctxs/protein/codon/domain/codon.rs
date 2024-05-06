use std::fmt::{self, Display};

use anyhow::ensure;

use crate::ctxs::{fasta::domain::fasta::C_RNA_BASES, protein::codon::domain::error::CodonError};

/// Codon Value Object
pub type CodonValue = [char; 3];

/// Codon encapsulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Codon {
    codon: CodonValue,
}

impl Display for Codon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codon.iter().collect::<String>())
    }
}

impl TryFrom<[char; 3]> for Codon {
    type Error = anyhow::Error;

    fn try_from(value: [char; 3]) -> Result<Self, Self::Error> {
        let value_binding = value;
        ensure!(
            value_binding.into_iter().all(|c| C_RNA_BASES.contains(&c)),
            CodonError::InvalidBases
        );
        Ok(Codon {
            codon: value_binding,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::protein::codon::domain::codon::Codon;

    #[test]
    fn should_create_a_codon() {
        let codon: Codon = Codon::try_from(['a', 'u', 'g']).unwrap();
        assert_eq!(codon.to_string(), "aug".to_string())
    }

    #[test]
    #[should_panic]
    fn should_panic_on_invalid_base() {
        let _c = match Codon::try_from(['a', 'u', 't']) {
            Err(_) => panic!(),
            Ok(x) => x,
        };
    }
}
