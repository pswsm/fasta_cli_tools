//! This library contains the structs for Aminoacid and ProteinChain
use std::fmt::Display;

use anyhow::ensure;

use crate::utils::AMINOACID_TABLE;

pub type CodonValue = [char; 3];

impl<T: ToString> From<T> for CodonValue {
    fn from(value: T) -> Self {
        let codon: CodonValue = value.to_string();
        ensure!(codon.len() <= 3);
        codon.as_slice()
    }
}

pub type AminoacidValue = char;

/// Representation a protein or aminoacid
#[derive(Default, Clone)]
pub struct Aminoacid {
    /// Letter of the protein
    pub aminoacid: AminoacidValue,

    /// Codons of RNA that can compile this protein
    pub codons: Vec<CodonValue>,
}

/// Transforms a tuple of string slices into an Aminoacid struct. The first slice from said vector
/// will be the protein letter. The following elements will be it's codons.
///
/// This allows for the creation od custom aminoacids.
///
/// For example:
/// ```
/// let aa: Aminoacid = Aminoacid::from(vec!["m", "aug"]);
///
/// println!("{}: {}", aa.aa, aa.codons) // Will output "m: aug"
/// ```
impl From<(AminoacidValue, Vec<CodonValue>)> for Aminoacid {
    fn from((aminoacid, codons): (AminoacidValue, Vec<CodonValue>)) -> Self {
        Aminoacid { aminoacid, codons }
    }
}

/// From a given codon, returns the corresponding aminoacid
impl From<CodonValue> for Aminoacid {
    fn from(value: CodonValue) -> Self {
        AMINOACID_TABLE
            .into_iter()
            .filter(|aminoacid| aminoacid.codons.iter().any(|aa_codon| aa_codon == &value))
            .collect::<Vec<Aminoacid>>()[0]
            .clone()
    }
}

/// Struct representing a protein: a chain of aminoacids
pub struct Protein {
    /// chain: A Vector holding Aminoacid structs
    pub chain: std::vec::Vec<Aminoacid>,
}

/// From a vector of Aminoacid structs create a new ProteinChain struct. The field "chain" becomes a copy of the passed vector
///
/// For example:
/// ```
/// let aminoacids: &[Aminoacid] = &[Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"])];
/// let proteins: ProteinChain = ProteinChain::from(aminoacids);
///
/// println!(proteins); // Will output "mmmm"
/// ```
impl From<std::vec::Vec<Aminoacid>> for Protein {
    fn from(aminoacids: std::vec::Vec<Aminoacid>) -> Self {
        Protein { chain: aminoacids }
    }
}

impl Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.chain
                .iter()
                .map(|aa| aa.aminoacid.clone())
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::Aminoacid;
    use crate::structs::Protein;
    #[test]
    fn protein_create() {
        let aminoacids: std::vec::Vec<Aminoacid> = vec![
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
        ];
        let proteins: Protein = Protein::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }

    #[test]
    fn aminoacid_create() {
        let methionine: Aminoacid = Aminoacid::from(('m', vec![['a', 'u', 'g']]));
        assert_eq!(
            methionine.aminoacid == 'm',
            methionine.codons == vec![['a', 'u', 'g']]
        )
    }
}
