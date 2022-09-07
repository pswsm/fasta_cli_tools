use core::fmt;
use std::fmt::Display;

struct Aminoacid {
    aa: char,
    codons: std::vec::Vec<String>,
}

impl Aminoacid {
    const fn from(aa_char: char, pos_codons: std::vec::Vec<String>) -> Aminoacid {
        Aminoacid {
            aa: aa_char,
            codons: pos_codons,
        }
    }
}

impl Display for Aminoacid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n{:#?}", self.aa, self.codons)
    }
}

#[cfg(test)]
mod tests {
    use crate::Aminoacid;

    #[test]
    fn test_struct() {
        let methionine: Aminoacid = Aminoacid::from('m', vec!["aug".to_string()]);
        assert_eq!(
            methionine.aa == 'm',
            methionine.codons == vec!["aug".to_string()]
        )
    }
}
