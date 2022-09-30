use core::fmt;
use std::{collections::HashMap, fmt::Display};

#[derive(Default)]
pub struct Aminoacid {
	aa: String,
	codons: std::vec::Vec<String>,
}

impl Aminoacid {
	pub fn from(aa_char: &str, pos_codons: std::vec::Vec<&str>) -> Aminoacid {
		Aminoacid {
			aa: aa_char.to_owned(),
			codons: pos_codons.into_iter().map(|codon| codon.to_owned()).collect(),
		}
	}
	pub const fn new() -> Aminoacid {
		Aminoacid {
			aa: String::new(),
			codons: Vec::new(),
		}
	}
    pub fn push_codon(&mut self, codon: &str) {
        self.codons.push(codon.to_owned());
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
		(String::from("gag"), String::from("e")),
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
		(String::from("cgu"), String::from("r")),
		(String::from("cgc"), String::from("r")),
		(String::from("cga"), String::from("r")),
		(String::from("cgg"), String::from("r")),
		(String::from("aga"), String::from("r")),
		(String::from("agg"), String::from("r")),
		(String::from("ucu"), String::from("s")),
		(String::from("ucc"), String::from("s")),
		(String::from("uca"), String::from("s")),
		(String::from("ucg"), String::from("s")),
		(String::from("agu"), String::from("s")),
		(String::from("agc"), String::from("s")),
		(String::from("guu"), String::from("v")),
		(String::from("guc"), String::from("v")),
		(String::from("gua"), String::from("v")),
		(String::from("gug"), String::from("v")),
		(String::from("ugg"), String::from("w")),
		(String::from("uau"), String::from("y")),
		(String::from("uac"), String::from("y")),
		(String::from("uaa"), String::from("*")),
		(String::from("uag"), String::from("*")),
		(String::from("uga"), String::from("*")),
	]);
	return aa_map;
}

pub fn dna2aa(data: fasta::Fasta, uppercase: bool) -> Vec<String> {
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
    let result: Vec<String> = match uppercase {
        true => aa_seq.to_uppercase().split_inclusive("*").map(|s| String::from(s)).collect(),
        false => aa_seq.split_inclusive("*").map(|s| String::from(s)).collect()
    };
    result
}

pub fn dna2aa_str(data: fasta::Fasta, uppercase: bool) -> String {
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
    let result: Vec<String> = match uppercase {
        true => aa_seq.to_uppercase().split_inclusive("*").map(|s| String::from(s)).collect(),
        false => aa_seq.split_inclusive("*").map(|s| String::from(s)).collect()
    };
    result.join("\n")
}

#[cfg(test)]
mod tests {
	use crate::{make_aa_hash_table, Aminoacid, dna2aa, dna2aa_str};

	#[test]
	fn test_struct() {
		let methionine: Aminoacid = Aminoacid::from("m", vec!["aug"]);
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
			table.get(&"uga".to_string()) == Some(&String::from("*"))
		)
	}

	#[test]
	fn test_dna2aa_upper() {
		let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augaggcgauga".to_string() };
		let aa_sequence: Vec<String> = dna2aa(ff, true);
		assert_eq!(aa_sequence, vec!["MRR*".to_string()])
	}

	#[test]
	fn test_dna2aa_lower() {
		let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augaggcgauga".to_string() };
		let aa_sequence: Vec<String> = dna2aa(ff, false);
		assert_eq!(aa_sequence, vec!["mrr*".to_string()])
    }

	#[test]
	fn test_dna2aa_multiple() {
		let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augaggcgaugaaugaggcgauga".to_string() };
		let aa_sequence: Vec<String> = dna2aa(ff, false);
		assert_eq!(aa_sequence, vec!["mrr*".to_string(), "mrr*".to_string()])
	}

	#[test]
	fn test_dna2aa_upper_str() {
		let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augaggcgauga".to_string() };
		let aa_sequence: String = dna2aa_str(ff, true);
		assert_eq!(aa_sequence, "MRR*".to_string())
	}

	#[test]
	fn test_dna2aa_lower_str() {
		let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augaggcgauga".to_string() };
		let aa_sequence: String = dna2aa_str(ff, false);
		assert_eq!(aa_sequence, "mrr*".to_string())
    }

	#[test]
	fn test_dna2aa_multiple_str() {
		let ff: fasta::Fasta = fasta::Fasta { header: "".to_string(), sequence: "augaggcgaugaaugaggcgauga".to_string() };
		let aa_sequence: String = dna2aa_str(ff, false);
		assert_eq!(aa_sequence, "mrr*\nmrr*".to_string())
	}
}
