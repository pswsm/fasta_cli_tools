//! This library contains the structs for Aminoacid and ProteinChain
use std::fmt::{self, Display};

use crate::utils::AMINOACID_TABLE;

/// Basic codon value representation
pub type CodonValue = [char; 3];

/// Wrapper for CodonValue
#[derive(Debug, Clone, PartialEq)]
pub struct Codon {
    codon: CodonValue,
}

impl Display for Codon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codon.iter().collect::<String>())
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

impl Aminoacid {
    /// Gets the desired aminoacid from a codon
    pub fn get_aminoacd_from_codon(codon: Codon) -> Self {
        let found_aminoacid: Vec<Aminoacid> = AMINOACID_TABLE
            .clone()
            .into_iter()
            .filter(|aminoacid| aminoacid.codons.contains(&codon))
            .collect();
        match found_aminoacid.len() {
            0 => AMINOACID_TABLE[0].clone(),
            _ => found_aminoacid[0].clone(),
        }
    }
}

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

/// From a given codon, returns the corresponding aminoacid
impl From<Codon> for Aminoacid {
    fn from(value: Codon) -> Self {
        AMINOACID_TABLE
            .clone()
            .into_iter()
            .filter(|aminoacid| aminoacid.codons.contains(&value))
            .collect::<Vec<Aminoacid>>()[0]
            .clone()
    }
}

/// Struct representing a protein: a chain of aminoacids
pub struct Protein {
    /// chain: A Vector holding Aminoacid structs
    pub chain: Vec<Aminoacid>,
}

/// From a vector of [crate::Aminoacid] create a new Protein.
///
/// For example:
/// ```
/// let aminoacids: Vec<Aminoacid> = vec![Aminoacid::from(('m', vec![Codon::from_chars('a', 'u', 'g')])), Aminoacid::from('m', vec![Codon::from_chars('a', 'u', 'g')])];
/// let proteins: Protein = Protein::from(aminoacids);
/// ```
impl From<Vec<Aminoacid>> for Protein {
    fn from(aminoacids: std::vec::Vec<Aminoacid>) -> Self {
        Protein { chain: aminoacids }
    }
}

impl Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.chain.iter().map(|aa| aa.aminoacid).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::Aminoacid;
    use crate::structs::Codon;
    use crate::structs::Protein;
    #[test]
    fn structs_protein() {
        let aminoacids: std::vec::Vec<Aminoacid> = vec![
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
        ];
        let proteins: Protein = Protein::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }

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
    fn structs_codon() {
        let codon: Codon = Codon::from_chars(['a', 'u', 'g']);
        assert_eq!(codon.to_string(), "aug".to_string())
    }

    #[test]
    fn get_from_codon() {
        let matched_aa: Aminoacid =
            Aminoacid::get_aminoacd_from_codon(Codon::from_chars(['a', 'u', 'g']));
        let base_aa: Aminoacid = Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])]));
        assert_eq!(matched_aa.aminoacid, base_aa.aminoacid);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }
}
