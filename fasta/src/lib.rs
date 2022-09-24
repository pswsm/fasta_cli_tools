use std::vec::Vec;

pub const DNA_BASES: [&str; 4] = ["a", "t", "c", "g"];
pub const RNA_BASES: [&str; 4] = ["a", "u", "c", "g"];

#[derive(Clone)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}

impl Fasta {
    #[allow(dead_code)]
    pub const fn new() -> Fasta {
        Fasta {
            header: String::new(),
            sequence: String::new(),
        }
    }

    pub const fn from(headr: String, seqnc: String) -> Fasta {
        Fasta {
            header: headr,
            sequence: seqnc,
        }
    }

    pub fn to_string(&self) -> String {
        String::from(format!("{}\n{}", self.header, self.sequence))
    }

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
}
