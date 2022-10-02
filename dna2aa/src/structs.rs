//! This module contains the structs for Aminoacid and ProteinChain
/// Module aminoacid contains the Aminoacid struct, the Aminoacids enum (unfinished) and its
/// implementations. The trait From is a work in progress
pub mod aminoacid {
    use core::fmt;
    use std::fmt::Display;
    use std::vec;

    #[allow(dead_code)]
    pub enum Aminoacids {
        Met(Aminoacid)
    }

    #[derive(Default, Clone)]
    pub struct Aminoacid {
        pub aa: String,
        pub codons: std::vec::Vec<String>,
    }

    impl Aminoacid {
        /// Manual constructor for aminoacid
        /// This function will be deprecated and removed once the trait From<T> is implemented
        /// correctly. At the moment use this to create Aminoacid structs.
        ///
        /// aa_char = aminoacid letter -> cehck if exists in enum [WIP]
        /// pos_codons = codons translating to said protein
        pub fn from_manual(aa_char: &str, pos_codons: &[&str]) -> Aminoacid {
            Aminoacid {
                aa: aa_char.to_string(),
                codons: pos_codons.iter().map(|codon| codon.to_string()).collect(),
            }
        }
    }

    /// Technically, it should work same as creating a ProteinChain from an array, but it won't.
    ///
    /// From an array of Strings or &str (still undecided):
    ///     -> array[0] = aminoacid letter -> check if exists in enum
    ///     -> array[1..end] = codons translating to said protein
    ///
    /// THIS DOES NOT WORK, use public function from_manual() instead [see above]
    impl From<&[String]> for Aminoacid {
        fn from(data: &[String]) -> Self {
            let data: vec::Vec<String> = data.to_vec().iter().map(|e| e.to_string()).collect();
            let split_data: (&[String], &[String]) = data.split_at(1);
            Aminoacid { aa: split_data.0[0].to_string(), codons: split_data.1.to_vec() }
        }
    }

    impl Display for Aminoacid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}:\n{:#?}", self.aa, self.codons)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::structs::aminoacid::Aminoacid;
        #[test]
        fn from_slice() {
            let methionine: Aminoacid = Aminoacid::from_manual("m", &["aug"]);
            assert_eq!(
                methionine.aa == "m",
                methionine.codons == vec!["aug".to_string()]
            )
        }
    }
}
/// Module "proteins" contains struct ProteinChain, and it's traits and implementations
pub mod proteins {
    use core::fmt;
    use std::fmt::Display;
    use crate::structs::aminoacid::Aminoacid;
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
        use crate::structs::{proteins::ProteinChain, aminoacid::Aminoacid};
        #[test]
        fn from_slice() {
            let aminoacids: &[Aminoacid] = &[Aminoacid::from_manual("m", &["aug"]), Aminoacid::from_manual("m", &["aug"]), Aminoacid::from_manual("m", &["aug"]), Aminoacid::from_manual("m", &["aug"])];
            let proteins: ProteinChain = ProteinChain::from(aminoacids);
            assert_eq!(proteins.to_string(), "mmmm".to_string())
        }

        #[test]
        fn from_vector() {
            let aminoacids: std::vec::Vec<Aminoacid> = vec![Aminoacid::from_manual("m", &["aug"]), Aminoacid::from_manual("m", &["aug"]), Aminoacid::from_manual("m", &["aug"]), Aminoacid::from_manual("m", &["aug"])];
            let proteins: ProteinChain = ProteinChain::from(aminoacids);
            assert_eq!(proteins.to_string(), "mmmm".to_string())
        }
    }
}
