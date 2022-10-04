//! Fasta file representation and basic methods for it
use std::vec::Vec;
use core::fmt;
use std::fmt::Display;

/// DNA allowed bases
pub const DNA_BASES: [&str; 4] = ["a", "t", "c", "g"];
/// RNA allowed bases
pub const RNA_BASES: [&str; 4] = ["a", "u", "c", "g"];

/// `.fasta` file representation in Rust. It has a header, and a sequence.
#[derive(Clone)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}

impl Display for Fasta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "> {}\n{}", self.header, self.sequence)
    }
}

impl Fasta {
    /// Creates a new, empty Fasta.
    #[allow(dead_code)]
    pub const fn new() -> Fasta {
        Fasta {
            header: String::new(),
            sequence: String::new(),
        }
    }
    
    /// Creates a new Fasta from a header, and a sequence.
    pub const fn from(headr: String, seqnc: String) -> Fasta {
        Fasta {
            header: headr,
            sequence: seqnc,
        }
    }

    /// Creates a new fasta with the complementary chain of `self`.
    pub fn complement(&self) -> Fasta {
        let og_seq: Vec<char> = self.sequence.chars().collect();
        let comp_vec: Vec<String> = {
            let mut cs: Vec<&str> = Vec::new();
            let is_rna: bool = og_seq.contains(&'u');
            if is_rna {
                for b in og_seq.into_iter() {
                    match b {
                        'a' => cs.append(&mut vec!["u"]),
                        'u' => cs.append(&mut vec!["a"]),
                        'c' => cs.append(&mut vec!["g"]),
                        'g' => cs.append(&mut vec!["c"]),
                        _ => (),
                    };
                }
            } else {
                for b in og_seq.into_iter() {
                    match b {
                        'a' => cs.append(&mut vec!["t"]),
                        't' => cs.append(&mut vec!["a"]),
                        'c' => cs.append(&mut vec!["g"]),
                        'g' => cs.append(&mut vec!["c"]),
                        _ => (),
                    };
                }
            }
            cs.into_iter().map(|b| b.to_string()).collect()
        };
        let comp_sequence: String = comp_vec.into_iter().map(|b| b).collect();
        Fasta {
            header: format!(">Complementary of {}", self.header.clone()),
            sequence: comp_sequence,
        }
    }

    /// Creates a new `Fasta` with the reverse chain of `self`.
    pub fn reverse(&self) -> Fasta {
        let rev_seq: String = self.sequence.chars().rev().collect();
        Fasta {
            header: format!(">Reverse of {}", self.header.clone()),
            sequence: rev_seq,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Fasta;

    #[test]
    fn make_fasta() {
        let fasta: Fasta = Fasta::from(String::from("> Test Header"), String::from("atcg"));
        assert_eq!(
            fasta.header == String::from("> Test Header"),
            fasta.sequence == String::from("atcg")
        )
    }

    #[test]
    fn make_reverse() {
        let fasta: Fasta = Fasta::from(String::from("> Test Header"), String::from("atcg"));
        assert_eq!(
            fasta.header == String::from("> Reverse of > Test Header"),
            fasta.sequence == String::from("gcta")
        )
    }

    #[test]
    fn make_complement_dna() {
        let fasta: Fasta = Fasta { header: String::from("> Test Header"), sequence: String::from("atcg") };
        assert_eq!(
            fasta.header == String::from("> Complementary of > Test header"),
            fasta.sequence == String::from("tagc")
        )
    }

    #[test]
    fn make_complement_rna() {
        let fasta: Fasta = Fasta { header: String::from("> Test Header"), sequence: String::from("aucg") };
        assert_eq!(
            fasta.header == String::from("> Complementary of > Test header"),
            fasta.sequence == String::from("uagc")
        )
    }
}
