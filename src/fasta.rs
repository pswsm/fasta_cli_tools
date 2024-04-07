//! Fasta file representation and basic methods for it
use core::fmt;
use std::{fmt::Display, str::Chars};

/// DNA allowed bases
pub const DNA_BASES: [&str; 4] = ["a", "t", "c", "g"];
/// RNA allowed bases
pub const RNA_BASES: [&str; 4] = ["a", "u", "c", "g"];

#[derive(Clone, PartialEq, Default)]
pub(crate) struct FastaHeader {
    header: String,
}

impl From<String> for FastaHeader {
    fn from(value: String) -> Self {
        FastaHeader {
            header: value.to_string(),
        }
    }
}

impl Display for FastaHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "> {}", self.header)
    }
}

#[derive(Clone, PartialEq)]
pub struct FastaSequence {
    sequence: String,
}

impl Display for FastaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.sequence)
    }
}

impl From<String> for FastaSequence {
    fn from(value: String) -> Self {
        FastaSequence { sequence: value }
    }
}

impl FastaSequence {
    pub fn get_chars(&self) -> Chars {
        self.sequence.chars()
    }
}

/// `.fasta` file representation in Rust. It has a header, and a sequence.
#[derive(Clone)]
pub struct Fasta {
    pub header: FastaHeader,
    pub sequence: FastaSequence,
}

impl Display for Fasta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.header, self.sequence)
    }
}

impl<T: ToString> From<(T, T)> for Fasta {
    fn from((header, sequence): (T, T)) -> Fasta {
        Fasta {
            header: FastaHeader::from(header.to_string()),
            sequence: FastaSequence::from(sequence.to_string()),
        }
    }
}

impl Fasta {
    /// Returns a new fasta with the complementary chain of `self`.
    pub fn complement(&self) -> Fasta {
        let original_sequence: Vec<char> = self.sequence.sequence.chars().collect();
        let comp_vec: Vec<String> = match original_sequence.contains(&'u') {
            true => original_sequence
                .iter()
                .map(|b| match b {
                    'a' => "u".to_string(),
                    'u' => "a".to_string(),
                    'c' => "g".to_string(),
                    'g' => "c".to_string(),
                    _ => "".to_string(),
                })
                .collect(),
            false => original_sequence
                .iter()
                .map(|b| match b {
                    'a' => "t".to_string(),
                    't' => "a".to_string(),
                    'c' => "g".to_string(),
                    'g' => "c".to_string(),
                    _ => "".to_string(),
                })
                .collect(),
        };
        let comp_sequence: String = comp_vec.into_iter().collect();
        Fasta {
            header: FastaHeader::from(format!("Complementary of {}", self.header.clone())),
            sequence: FastaSequence::from(comp_sequence),
        }
    }

    /// Creates a new `Fasta` with the reverse chain of `self`.
    pub fn reverse(&self) -> Fasta {
        let rev_seq: String = self.sequence.sequence.chars().rev().collect();
        Fasta {
            header: FastaHeader::from(format!("Reverse of {}", self.header.clone())),
            sequence: FastaSequence::from(rev_seq),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fasta::{Fasta, FastaHeader, FastaSequence};

    #[test]
    fn make_fasta_array() {
        let fasta: Fasta = Fasta::from(("test header", "atcg"));
        assert_eq!(
            fasta.header == FastaHeader::from("> test header".to_string()),
            fasta.sequence == FastaSequence::from("atcg".to_string())
        );
    }

    #[test]
    fn make_reverse() {
        let fasta: Fasta = Fasta::from(("test header", "atcg"));
        assert_eq!(
            fasta.header == FastaHeader::from("Reverse of > test Header".to_string()),
            fasta.sequence == FastaSequence::from("gcta".to_string())
        )
    }

    #[test]
    fn make_complement_dna() {
        let fasta: Fasta = Fasta::from(("test header", "atcg"));
        assert_eq!(
            fasta.header == FastaHeader::from("Complementary of > test header".to_string()),
            fasta.sequence == FastaSequence::from("tagc".to_string())
        )
    }

    #[test]
    fn make_complement_rna() {
        let fasta: Fasta = Fasta::from(("test header", "atcg"));
        assert_eq!(
            fasta.header == FastaHeader::from("Complementary of > test header".to_string()),
            fasta.sequence == FastaSequence::from("uagc".to_string())
        )
    }
}
