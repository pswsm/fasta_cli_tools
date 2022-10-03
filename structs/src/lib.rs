//! This library contains the structs for Aminoacid and ProteinChain
use std::vec;
use core::fmt;
use std::fmt::Display;

#[allow(dead_code)]
pub enum Aminoacids {
    Met(Aminoacid)
}

/// Struct representing a protein or aminoacid
///     -> aa: Letter of the protein
///     -> codons: Codons of RNA that can compile this protein
#[derive(Default, Clone)]
pub struct Aminoacid {
    pub aa: String,
    pub codons: std::vec::Vec<String>,
}

/// From a vector of &str:
///     -> vec[0]: aminoacid letter -> check if exists in enum
///     -> vec[1..end]: codons translating to said protein
impl From<std::vec::Vec<&str>> for Aminoacid {
    fn from(data: std::vec::Vec<&str>) -> Self {
        let data: vec::Vec<String> = data.to_vec().iter().map(|e| e.to_string()).collect();
        let split_data: (&[String], &[String]) = data.split_at(1);
        Aminoacid { aa: split_data.0[0].to_string(), codons: split_data.1.to_vec() }
    }
}

/// From a vector of String:
///     -> vec[0]: aminoacid letter -> check if exists in enum
///     -> vec[1..end]: codons translating to said protein
impl From<std::vec::Vec<String>> for Aminoacid {
    fn from(data: std::vec::Vec<String>) -> Self {
        let data: vec::Vec<String> = data.to_vec().iter().map(|e| e.to_string()).collect();
        let split_data: (&[String], &[String]) = data.split_at(1);
        Aminoacid { aa: split_data.0[0].to_string(), codons: split_data.1.to_vec() }
    }
}

/// Struct representing a chain of proteins:
///     -> chain: A Vector holding many Aminoacid structs
#[derive(Default)]
pub struct ProteinChain {
    pub chain: std::vec::Vec<Aminoacid>
}

/// From array of Aminoacid structs create a new ProteinChain struct
/// Since it comes from an array it's converted to a Vector (std::vec::Vec)
impl From<&[Aminoacid]> for ProteinChain {
    fn from(some_aa: &[Aminoacid]) -> Self {
        ProteinChain { chain: some_aa.to_vec() }
    }
}

/// From a vector of Aminoacid structs create a new ProteinChain struct
/// The vector from which chain is created is a different one from the previously used
impl From<std::vec::Vec<Aminoacid>> for ProteinChain {
    fn from(some_aa: std::vec::Vec<Aminoacid>) -> Self {
        ProteinChain { chain: some_aa.to_vec() }
    }
}

/// The implementation of the Display trait for ProteinChain
/// Writes a string made of each aminoacid letter [Aminoacid.aa]
impl Display for ProteinChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chain.iter().map(|aa| aa.aa.clone()).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use crate::ProteinChain;
    use crate::Aminoacid;
    #[test]
    fn from_array() {
        let aminoacids: &[Aminoacid] = &[Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"])];
        let proteins: ProteinChain = ProteinChain::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }

    #[test]
    fn from_vector() {
        let aminoacids: std::vec::Vec<Aminoacid> = vec![Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"])];
        let proteins: ProteinChain = ProteinChain::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }
    #[test]
    fn from_vec_str() {
        let methionine: Aminoacid = Aminoacid::from(vec!["m", "aug"]);
        assert_eq!(
            methionine.aa == "m".to_owned(),
            methionine.codons == vec!["aug".to_owned()]
            )
    }

    #[test]
    fn from_vec_string() {
        let methionine: Aminoacid = Aminoacid::from(vec!["m".to_owned(), "aug".to_owned()]);
        assert_eq!(
            methionine.aa == "m".to_owned(),
            methionine.codons == vec!["aug".to_owned()]
            )
    }
}
