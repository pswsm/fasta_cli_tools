use std::fmt::Display;

use crate::ctxs::codon::domain::codon::Codon;
use crate::shared::utils::AMINOACID_TABLE;

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
/// Aminoacid::from(('m', vec![Codon::from_chars('a', 'u', 'g')]));
/// ```
/// or
/// ```
/// Aminoacid::from(('c', vec![Codon::from_chars(['u', 'g', 'u']), Codon::from_chars(['u', 'g', 'c'])])),
/// ```
impl From<(AminoacidValue, Vec<Codon>)> for Aminoacid {
    fn from((aminoacid, codons): (AminoacidValue, Vec<Codon>)) -> Self {
        Aminoacid { aminoacid, codons }
    }
}

impl From<[char; 3]> for Aminoacid {
    fn from(value: [char; 3]) -> Self {
        let codon: Codon = Codon::from_chars(value);
        Aminoacid::from(codon)
    }
}

/// From a given codon, returns the corresponding aminoacid
impl From<Codon> for Aminoacid {
    fn from(value: Codon) -> Self {
        let aminoacid: &Aminoacid = &AMINOACID_TABLE
            .clone()
            .into_iter()
            .filter(|aminoacid| aminoacid.codons.contains(&value))
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
    fn structs_aminoacid() {
        let methionine: Aminoacid =
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])]));
        assert_eq!(
            methionine.aminoacid == 'm',
            methionine.codons == vec![Codon::from_chars(['a', 'u', 'g'])]
        )
    }

    #[test]
    fn get_from_codon() {
        let matched_aa: Aminoacid = Aminoacid::from(['a', 'u', 'g']);
        let base_aa: Aminoacid = Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])]));
        assert_eq!(matched_aa.aminoacid, base_aa.aminoacid);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }
}
