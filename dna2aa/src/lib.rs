use core::fmt;
use std::{collections::HashMap, fmt::Display};

#[derive(Default)]
pub struct Aminoacid {
    aa: String,
    codons: std::vec::Vec<String>,
}

impl Aminoacid {
    pub const fn from(aa_char: String, pos_codons: std::vec::Vec<String>) -> Aminoacid {
        Aminoacid {
            aa: aa_char,
            codons: pos_codons,
        }
    }
    pub const fn new() -> Aminoacid {
        Aminoacid {
            aa: String::new(),
            codons: Vec::new(),
        }
    }
}

impl Display for Aminoacid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n{:#?}", self.aa, self.codons)
    }
}

fn make_aa_hash_table() -> HashMap<String, String> {
    let aa_map = HashMap::from([
        (String::from("gcu"), String::from("a")),
        (String::from("gcc"), String::from("a")),
        (String::from("gca"), String::from("a")),
        (String::from("gcg"), String::from("a")),
        (String::from("ugu"), String::from("c")),
        (String::from("ugc"), String::from("c")),
        (String::from("gau"), String::from("d")),
        (String::from("gac"), String::from("d")),
        (String::from("gaa"), String::from("e")),
        (String::from("uuu"), String::from("f")),
        (String::from("uuc"), String::from("f")),
        (String::from("ggu"), String::from("g")),
        (String::from("ggc"), String::from("g")),
        (String::from("gga"), String::from("g")),
        (String::from("ggg"), String::from("g")),
        (String::from("cau"), String::from("h")),
        (String::from("cac"), String::from("h")),
        (String::from("auu"), String::from("i")),
        (String::from("auc"), String::from("i")),
        (String::from("aua"), String::from("i")),
        (String::from("aaa"), String::from("k")),
        (String::from("aag"), String::from("k")),
        (String::from("uua"), String::from("l")),
        (String::from("uug"), String::from("l")),
        (String::from("cuu"), String::from("l")),
        (String::from("cuc"), String::from("l")),
        (String::from("cua"), String::from("l")),
        (String::from("cug"), String::from("l")),
        (String::from("aug"), String::from("m")),
        (String::from("ccu"), String::from("n")),
        (String::from("ccc"), String::from("n")),
        (String::from("cca"), String::from("n")),
        (String::from("ccg"), String::from("n")),
        (String::from("caa"), String::from("p")),
        (String::from("cag"), String::from("p")),
        (String::from("cgu"), String::from("q")),
        (String::from("cgc"), String::from("q")),
        (String::from("cga"), String::from("q")),
        (String::from("cgg"), String::from("q")),
        (String::from("aga"), String::from("q")),
        (String::from("agg"), String::from("q")),
        (String::from("ucu"), String::from("r")),
        (String::from("ucg"), String::from("r")),
        (String::from("agu"), String::from("r")),
        (String::from("agc"), String::from("r")),
        (String::from("guu"), String::from("t")),
        (String::from("guc"), String::from("t")),
        (String::from("gua"), String::from("t")),
        (String::from("gug"), String::from("t")),
        (String::from("ugg"), String::from("v")),
        (String::from("uau"), String::from("w")),
        (String::from("uac"), String::from("w")),
        (String::from("uaa"), String::from("y")),
        (String::from("uag"), String::from("y")),
        (String::from("uga"), String::from("y")),
    ]);
    return aa_map;
}

pub fn dna2aa(data: fasta::Fasta) -> String {
    let rna_sequence_spl: Vec<String> = data.sequence.chars().map(|c| String::from(c)).collect();
    let aa_bases: HashMap<String, String> = make_aa_hash_table();
    let aa_seq = {
        let mut aa_seq_tmp: Vec<String> = Vec::new();
        for gidx in (0..(rna_sequence_spl.len())).step_by(3) {
            let group: String = vec![rna_sequence_spl[gidx].clone(), rna_sequence_spl[gidx+1].clone(), rna_sequence_spl[gidx+2].clone()].join("");
            aa_seq_tmp.push(aa_bases.get(&group).unwrap_or(&"-".to_string()).to_string());
        };
        aa_seq_tmp.join("")
    };
    return aa_seq
}

#[cfg(test)]
mod tests {
    use crate::{make_aa_hash_table, Aminoacid, dna2aa};

    #[test]
    fn test_struct() {
        let methionine: Aminoacid = Aminoacid::from("m".to_string(), vec!["aug".to_string()]);
        assert_eq!(
            methionine.aa == "m",
            methionine.codons == vec!["aug".to_string()]
        )
    }

    #[test]
    fn test_mk_table() {
        let table = make_aa_hash_table();
        assert_eq!(
            table.get(&"aug".to_string()) == Some(&String::from("m")),
            table.get(&"uga".to_string()) == Some(&String::from("y"))
        )
    }

    #[test]
    fn test_dna2aa() {
        let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augugaagu".to_string() };
        let aa_sequence: String = dna2aa(ff);
        assert_eq!(aa_sequence, "myr".to_string())
    }
}
