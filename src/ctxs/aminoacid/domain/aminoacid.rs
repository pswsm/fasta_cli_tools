use std::fmt::Display;

use crate::ctxs::{codon::domain::codon::Codon, shared::utils::AMINOACID_TABLE};

/// Basic aminoacid value representation
type AminoacidValue = char;

/// Representation a protein or aminoacid
#[derive(Default, Clone)]
pub struct Aminoacid {
    /// Letter of the protein
    pub aminoacid: AminoacidValue,

    /// Codons of RNA that can compile this protein
    pub codons: Vec<Codon>,
}

impl Display for Aminoacid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.aminoacid)
    }
}

/// Use this to create imaginary aminoacids.
/// Example:
/// ```
/// Aminoacid::from(('m', vec![['a', 'u', 'g']]));
/// ```
/// or
/// ```
/// Aminoacid::from(('c', vec![['u', 'g', 'u'], ['u', 'g', 'c']])),
/// ```
impl From<(AminoacidValue, Vec<[char; 3]>)> for Aminoacid {
    fn from(value: (AminoacidValue, Vec<[char; 3]>)) -> Self {
        let codons: Vec<Codon> = value.1.into_iter().map(Codon::from).collect();
        Aminoacid {
            aminoacid: value.0,
            codons,
        }
    }
}

impl From<[char; 3]> for Aminoacid {
    fn from(value: [char; 3]) -> Self {
        let codon: Codon = Codon::from_chars(value);
        let aminoacid = &AMINOACID_TABLE
            .clone()
            .into_iter()
            .filter(|aminoacid| aminoacid.codons.contains(&codon))
            .collect::<Vec<Aminoacid>>()[0];
        Aminoacid {
            aminoacid: aminoacid.aminoacid,
            codons: aminoacid.codons.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::{aminoacid::domain::aminoacid::Aminoacid, codon::domain::codon::Codon};

    #[test]
    fn test_aminoacid() {
        let methionine: Aminoacid = Aminoacid::from(('m', vec![['a', 'u', 'g']]));
        assert_eq!(
            methionine.aminoacid == 'm',
            methionine.codons == vec![Codon::from_chars(['a', 'u', 'g'])]
        )
    }

    #[test]
    fn get_from_codon() {
        let matched_aa: Aminoacid = Aminoacid::from(['a', 'u', 'g']);
        let base_aa: Aminoacid = Aminoacid::from(('m', vec![['a', 'u', 'g']]));
        assert_eq!(matched_aa.aminoacid, base_aa.aminoacid);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }
}
