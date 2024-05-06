use std::fmt::Display;

use anyhow::ensure;

use crate::ctxs::{
    protein::{aminoacid::domain::errors::AminoacidErrors, codon::domain::codon::Codon},
    shared::utils::AMINOACID_TABLE,
};

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

/// Use this implementation to create imaginary aminoacids.
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
        let codons: Vec<Codon> = value
            .1
            .into_iter()
            .map(|c| Codon::try_from(c).unwrap())
            .collect();
        Aminoacid {
            aminoacid: value.0,
            codons,
        }
    }
}

/// This is implememntaion is meant to be the main one to create aminoacids,
/// since it retrieves them from the aminoacid table and checks if it exists.
impl TryFrom<[char; 3]> for Aminoacid {
    type Error = anyhow::Error;
    fn try_from(value: [char; 3]) -> Result<Self, Self::Error> {
        let codon: Codon = Codon::try_from(value).unwrap();
        let aminoacid: &Vec<Aminoacid> = &AMINOACID_TABLE
            .clone()
            .into_iter()
            .filter(|aminoacid| aminoacid.codons.contains(&codon))
            .collect::<Vec<Aminoacid>>();
        ensure!(
            aminoacid.len() > 0,
            AminoacidErrors::NonExistingAminoacidError
        );
        Ok(Aminoacid {
            aminoacid: aminoacid[0].aminoacid,
            codons: aminoacid[0].codons.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::protein::{
        aminoacid::domain::aminoacid::Aminoacid, codon::domain::codon::Codon,
    };

    #[test]
    fn should_create_aminoacid_from_scratch() {
        let methionine: Aminoacid = Aminoacid::from(('m', vec![['a', 'u', 'g']]));
        assert_eq!(
            methionine.aminoacid == 'm',
            methionine.codons == vec![Codon::try_from(['a', 'u', 'g']).unwrap()]
        )
    }

    #[test]
    fn should_create_real_aminoacid() {
        let matched_aa: Aminoacid = Aminoacid::try_from(['a', 'u', 'g']).unwrap();
        let base_aa: Aminoacid = Aminoacid::try_from(('m', vec![['a', 'u', 'g']])).unwrap();
        assert_eq!(matched_aa.aminoacid, base_aa.aminoacid);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }

    #[test]
    #[should_panic]
    fn should_fail_to_create_nonexisting_aminoacid() {
        let _ = match Aminoacid::try_from(['a', 'c', 'g']) {
            Err(_) => panic!(),
            Ok(x) => x,
        };
    }
}
