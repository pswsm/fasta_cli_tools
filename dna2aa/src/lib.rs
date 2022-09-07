use core::fmt;
use std::fmt::Display;

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

pub fn make_aa_table() -> Vec<Aminoacid> {
    let aas: Vec<&str> = vec![
        "a", "c", "d", "e", "f", "g", "h", "i", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v",
        "w", "y",
    ];
    let aa_combs: Vec<Vec<String>> = vec![
        vec![
            String::from("gcu"),
            String::from("gcc"),
            String::from("gca"),
            String::from("gcg"),
        ],
        vec![String::from("ugu"), String::from("ugc")],
        vec![String::from("gau"), String::from("gac")],
        vec![String::from("gaa"), String::from("gac")],
        vec![String::from("uuu"), String::from("uuc")],
        vec![
            String::from("ggu"),
            String::from("ggc"),
            String::from("gga"),
            String::from("ggg"),
        ],
        vec![String::from("cau"), String::from("cac")],
        vec![
            String::from("auu"),
            String::from("auc"),
            String::from("aua"),
        ],
        vec![String::from("aaa"), String::from("aag")],
        vec![
            String::from("uua"),
            String::from("uug"),
            String::from("cuu"),
            String::from("cuc"),
            String::from("cua"),
            String::from("cug"),
        ],
        vec![String::from("aug")],
        vec![String::from("aau"), String::from("aac")],
        vec![
            String::from("ccu"),
            String::from("ccc"),
            String::from("cca"),
            String::from("ccg"),
        ],
        vec![String::from("caa"), String::from("cag")],
        vec![
            String::from("cgu"),
            String::from("cgc"),
            String::from("cga"),
            String::from("cgg"),
            String::from("aga"),
            String::from("agg"),
        ],
        vec![
            String::from("ucu"),
            String::from("ucc"),
            String::from("uca"),
            String::from("ucg"),
            String::from("agu"),
            String::from("agc"),
        ],
        vec![
            String::from("guu"),
            String::from("guc"),
            String::from("gua"),
            String::from("gug"),
        ],
        vec![String::from("ugg")],
        vec![String::from("uau"), String::from("uac")],
        vec![
            String::from("uaa"),
            String::from("uag"),
            String::from("uga"),
        ],
    ];
    let aa_holder: Vec<Aminoacid> = {
        let mut tmp_aa = vec![Aminoacid::new()];
        for idx in 0..aas.len() {
            tmp_aa.push(Aminoacid::from(aas[idx].to_string(), aa_combs[idx].clone()));
        }
        tmp_aa
    };
    aa_holder
}

pub fn dna2aa() -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::{make_aa_table, Aminoacid};

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
        let table = make_aa_table();
        assert_eq!(
            table[10].aa == String::from("m"),
            table[10].codons == vec!(String::from("aug"))
        )
    }
}
