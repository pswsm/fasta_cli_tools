//! I've put it here because it didn't work as a module inside dna2aa
//! This library contains the structs for Aminoacid and ProteinChain
use std::vec;
use core::fmt;
use std::fmt::Display;

#[allow(dead_code)]
pub enum Aminoacids {
    Met(Aminoacid)
}

/// Struct representing a protein or aminoacid
#[derive(Default, Clone)]
pub struct Aminoacid {
    /// aa: Letter of the protein
    pub aa: String,

    /// codons: Codons of RNA that can compile this protein
    pub codons: std::vec::Vec<String>,
}

/// Transforms a vector of string slices into an Aminoacid struct. The first slice from said vector
/// will be the protein letter. The following elements will be it's codons.
///
/// For example:
/// ```
/// let aa: Aminoacid = Aminoacid::from(vec!["m", "aug"]);
///
/// println!("{}: {}", aa.aa, aa.codons) // Will output "m: aug"
/// ```
impl From<std::vec::Vec<&str>> for Aminoacid {
    fn from(data: std::vec::Vec<&str>) -> Self {
        let data: vec::Vec<String> = data.to_vec().iter().map(|e| e.to_string()).collect();
        let split_data: (&[String], &[String]) = data.split_at(1);
        Aminoacid { aa: split_data.0[0].to_string(), codons: split_data.1.to_vec() }
    }
}

/// Transforms a vector of strings into an Aminoacid struct. The element from said vector
/// will be the protein letter. The following elements will be it's codons.
///
/// For example:
/// ```
/// let aa: Aminoacid = Aminoacid::from(vec!["m".to_string(), "aug".to_string()]);
///
/// println!("{}: {}", aa.aa, aa.codons); // Will output "m: aug"
/// ```
impl From<std::vec::Vec<String>> for Aminoacid {
    fn from(data: std::vec::Vec<String>) -> Self {
        let data: vec::Vec<String> = data.to_vec().iter().map(|e| e.to_string()).collect();
        let split_data: (&[String], &[String]) = data.split_at(1);
        Aminoacid { aa: split_data.0[0].to_string(), codons: split_data.1.to_vec() }
    }
}

/// Struct representing a chain of proteins
#[derive(Default)]
pub struct ProteinChain {
    /// chain: A Vector holding many Aminoacid structs
    pub chain: std::vec::Vec<Aminoacid>
}

/// From array of Aminoacid structs create a new ProteinChain struct.
/// Transform the array to a Vector (std::vec::Vec)
///
/// For example:
/// ```
/// let aminoacids: &[Aminoacid] = &[Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"])];
/// let proteins: ProteinChain = ProteinChain::from(aminoacids);
///
/// println!(proteins); // Will output "mmmm"
/// ```
impl From<&[Aminoacid]> for ProteinChain {
    fn from(some_aa: &[Aminoacid]) -> Self {
        ProteinChain { chain: some_aa.to_vec() }
    }
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
impl From<std::vec::Vec<Aminoacid>> for ProteinChain {
    fn from(some_aa: std::vec::Vec<Aminoacid>) -> Self {
        ProteinChain { chain: some_aa.to_vec() }
    }
}

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
    fn protein_from_array() {
        let aminoacids: &[Aminoacid] = &[Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"])];
        let proteins: ProteinChain = ProteinChain::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }

    #[test]
    fn protein_from_vector() {
        let aminoacids: std::vec::Vec<Aminoacid> = vec![Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"]), Aminoacid::from(vec!["m", "aug"])];
        let proteins: ProteinChain = ProteinChain::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }
    #[test]
    fn aa_from_vec_str() {
        let methionine: Aminoacid = Aminoacid::from(vec!["m", "aug"]);
        assert_eq!(
            methionine.aa == "m".to_owned(),
            methionine.codons == vec!["aug".to_owned()]
            )
    }

    #[test]
    fn aa_from_vec_string() {
        let methionine: Aminoacid = Aminoacid::from(vec!["m".to_owned(), "aug".to_owned()]);
        assert_eq!(
            methionine.aa == "m".to_owned(),
            methionine.codons == vec!["aug".to_owned()]
            )
    }
}
