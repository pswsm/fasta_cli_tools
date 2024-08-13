//! Fasta file representation and basic methods for it
use core::fmt;
use std::{fmt::Display, str::Chars};

use textwrap::fill;

/// DNA allowed bases
pub const DNA_BASES: [&str; 4] = ["a", "t", "c", "g"];
/// RNA allowed bases
pub const RNA_BASES: [&str; 4] = ["a", "u", "c", "g"];
/// DNA allowed bases
pub const C_DNA_BASES: [char; 4] = ['a', 't', 'c', 'g'];
/// RNA allowed bases
pub const C_RNA_BASES: [char; 4] = ['a', 'u', 'c', 'g'];

/// Object for the header of a fasta file
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

/// Object for the sequence
#[derive(Clone, PartialEq)]
pub struct FastaSequence {
    sequence: String,
}

impl Display for FastaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", fill(&self.sequence, 60))
    }
}

impl From<String> for FastaSequence {
    fn from(value: String) -> Self {
        FastaSequence {
            sequence: value.replace('\n', "").to_lowercase(),
        }
    }
}

impl FastaSequence {
    /// Returns the chars of the seuquence as Chars iterator
    pub fn get_chars(&self) -> Chars {
        self.sequence.chars()
    }

    /// Reverses itself
    fn reverse(&self) -> Self {
        FastaSequence {
            sequence: self.sequence.chars().rev().collect(),
        }
    }

    /// Complements itself
    fn complement_dna(&self) -> Self {
        FastaSequence {
            sequence: self
                .sequence
                .chars()
                .map(|b| match b {
                    'a' => "t".to_string(),
                    'u' => "a".to_string(),
                    'c' => "g".to_string(),
                    'g' => "c".to_string(),
                    _ => "".to_string(),
                })
                .collect(),
        }
    }

    /// Complements itself (for RNA)
    fn complement_rna(&self) -> Self {
        FastaSequence {
            sequence: self
                .sequence
                .chars()
                .map(|b| match b {
                    'a' => "u".to_string(),
                    'u' => "a".to_string(),
                    'c' => "g".to_string(),
                    'g' => "c".to_string(),
                    _ => "".to_string(),
                })
                .collect(),
        }
    }

    fn cut(&self, start: usize, end: usize) -> Self {
        FastaSequence {
            sequence: self.sequence.get(start..end).unwrap_or("").to_string(),
        }
    }

    fn to_uppercase(&self) -> Self {
        FastaSequence {
            sequence: self.sequence.to_uppercase(),
        }
    }
}

/// Fasta representation in Rust. It has a header, and a sequence.
#[derive(Clone)]
pub struct Fasta {
    /// The header
    pub header: FastaHeader,
    /// The sequence
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
    /// New fasta with it's complementary chain
    pub fn complement(&self) -> Self {
        let original_sequence: Vec<char> = self.sequence.sequence.chars().collect();
        let sequence: FastaSequence = match original_sequence.contains(&'u') {
            true => self.sequence.complement_rna(),
            false => self.sequence.complement_dna(),
        };
        Fasta {
            header: FastaHeader::from(format!("Complementary of {}", self.header.clone())),
            sequence,
        }
    }

    /// New fasta with it's reverse chain
    pub fn reverse(&self) -> Self {
        Fasta {
            header: FastaHeader::from(format!("Reverse of {}", self.header.clone())),
            sequence: self.sequence.reverse(),
        }
    }

    pub fn cut(&self, start: usize, end: usize) -> Self {
        Fasta {
            header: FastaHeader::from(format!("{}, cut {} - {}", self.header, start, end)),
            sequence: self.sequence.cut(start, end),
        }
    }

    pub fn uppercase(&self) -> Self {
        Fasta {
            header: self.header.to_owned(),
            sequence: self.sequence.to_uppercase(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::fasta::domain::fasta::{Fasta, FastaHeader, FastaSequence};

    #[test]
    fn make_fasta_array() {
        let fasta: Fasta = Fasta::from(("test header", "atcg"));
        assert_eq!(
            fasta.header == FastaHeader::from("test header".to_string()),
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
