//! Fasta file representation and basic methods for it
use core::fmt;
use std::fmt::Display;

/// DNA allowed bases
pub const DNA_BASES: [&str; 4] = ["a", "t", "c", "g"];
/// RNA allowed bases
pub const RNA_BASES: [&str; 4] = ["a", "u", "c", "g"];

/// `.fasta` file representation in Rust. It has a header, and a sequence.
#[derive(Default, Clone)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}

impl Display for Fasta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "> {}\n{}", self.header, self.sequence)
    }
}

impl From<&[&str; 2]> for Fasta {
    fn from(data: &[&str; 2]) -> Fasta {
        Fasta {
            header: data[0].to_string(),
            sequence: data[1].to_string(),
        }
    }
}

impl From<&[String; 2]> for Fasta {
    fn from(data: &[String; 2]) -> Fasta {
        Fasta {
            header: data[0].clone(),
            sequence: data[1].clone(),
        }
    }
}

impl From<&(&str, &str)> for Fasta {
    fn from(data: &(&str, &str)) -> Fasta {
        Fasta {
            header: data.0.to_string(),
            sequence: data.1.to_string(),
        }
    }
}

impl From<&(String, String)> for Fasta {
    fn from(data: &(String, String)) -> Fasta {
        Fasta {
            header: data.0.clone(),
            sequence: data.1.clone(),
        }
    }
}

impl Fasta {
    /// Returns a new fasta with the complementary chain of `self`.
    pub fn complement(&self) -> Fasta {
        let og_seq: Vec<char> = self.sequence.chars().collect();
        let comp_vec: Vec<String> = match og_seq.contains(&'u') {
            true => og_seq
                .iter()
                .map(|b| match b {
                    'a' => "u".to_string(),
                    'u' => "a".to_string(),
                    'c' => "g".to_string(),
                    'g' => "c".to_string(),
                    _ => "".to_string(),
                })
                .collect(),
            false => og_seq
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
            header: format!("Complementary of {}", self.header.clone()),
            sequence: comp_sequence,
        }
    }

    /// Creates a new `Fasta` with the reverse chain of `self`.
    pub fn reverse(&self) -> Fasta {
        let rev_seq: String = self.sequence.chars().rev().collect();
        Fasta {
            header: format!("Reverse of {}", self.header.clone()),
            sequence: rev_seq,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fasta::Fasta;

    #[test]
    fn make_fasta_array() {
        let fasta: Fasta = Fasta::from(&["test header", "atcg"]);
        assert_eq!(
            fasta.header == *"> test header".to_owned(),
            fasta.sequence == "atcg"
        );
    }

    #[test]
    fn make_reverse() {
        let fasta: Fasta = Fasta::from(&["test header", "atcg"]);
        assert_eq!(
            fasta.header == *String::from("Reverse of > test Header"),
            fasta.sequence == *String::from("gcta")
        )
    }

    #[test]
    fn make_complement_dna() {
        let fasta: Fasta = Fasta::from(&["test header", "atcg"]);
        assert_eq!(
            fasta.header == *String::from("Complementary of > test header"),
            fasta.sequence == *String::from("tagc")
        )
    }

    #[test]
    fn make_complement_rna() {
        let fasta: Fasta = Fasta::from(&["test header", "atcg"]);
        assert_eq!(
            fasta.header == *String::from("Complementary of > test header"),
            fasta.sequence == *String::from("uagc")
        )
    }
}
