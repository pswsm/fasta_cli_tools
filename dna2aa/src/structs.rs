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
        pub fn from_manual(aa_char: &str, pos_codons: &[&str]) -> Aminoacid {
            Aminoacid {
                aa: aa_char.to_string(),
                codons: pos_codons.iter().map(|codon| codon.to_string()).collect(),
            }
        }
    }

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
            let methionine: Aminoacid = Aminoacid::from(&["m".to_owned(), "aug".to_owned()]);
            assert_eq!(
                methionine.aa == "m",
                methionine.codons == vec!["aug".to_string()]
            )
        }
    }
}
pub mod proteins {
    use core::fmt;
    use std::fmt::Display;
    use crate::structs::aminoacid::Aminoacid;
    #[derive(Default)]
    pub struct ProteinChain {
        pub chain: std::vec::Vec<Aminoacid>
    }

    impl From<&[Aminoacid]> for ProteinChain {
        fn from(some_aa: &[Aminoacid]) -> Self {
            ProteinChain { chain: some_aa.to_vec() }
        }
    }

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
        use crate::structs::{proteins::ProteinChain, aminoacid::Aminoacid};
        #[test]
        fn from_slice() {
            let aminoacids: &[Aminoacid] = &[Aminoacid::from("m", &["aug"]), Aminoacid::from("m", &["aug"]), Aminoacid::from("m", &["aug"]), Aminoacid::from("m", &["aug"])];
            let proteins: ProteinChain = ProteinChain::from(aminoacids);
            assert_eq!(proteins.to_string(), "mmmm".to_string())
        }

        #[test]
        fn from_vector() {
            let aminoacids: std::vec::Vec<Aminoacid> = vec![Aminoacid::from("m", &["aug"]), Aminoacid::from("m", &["aug"]), Aminoacid::from("m", &["aug"]), Aminoacid::from("m", &["aug"])];
            let proteins: ProteinChain = ProteinChain::from(aminoacids);
            assert_eq!(proteins.to_string(), "mmmm".to_string())
        }
    }
}
