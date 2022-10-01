pub mod structs {
    use core::fmt;
    use std::fmt::Display;

    #[derive(Default, Clone)]
    pub struct Aminoacid {
        pub aa: String,
        pub codons: std::vec::Vec<String>,
    }

    impl Aminoacid {
        pub fn from(aa_char: &str, pos_codons: &[&str]) -> Aminoacid {
            Aminoacid {
                aa: aa_char.to_string(),
                codons: pos_codons.iter().map(|codon| codon.to_string()).collect(),
            }
        }
    }

    impl Display for Aminoacid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}:\n{:#?}", self.aa, self.codons)
        }
    }

    #[derive(Default)]
    pub struct ProteinChain {
        pub chain: std::vec::Vec<Aminoacid>
    }

    impl From<&[Aminoacid]> for ProteinChain {
        fn from(some_aa: &[Aminoacid]) -> Self {
            ProteinChain { chain: some_aa.to_vec() }
        }
    }

    impl Display for ProteinChain {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.chain.iter().map(|aa| aa.aa.clone()).collect::<String>())
        }
    }
}
