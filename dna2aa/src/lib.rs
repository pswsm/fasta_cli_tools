use core::fmt;
use std::{collections::HashMap, fmt::Display};

#[derive(Default, Clone)]
pub struct Aminoacid {
	aa: String,
	codons: std::vec::Vec<String>,
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
    chain: std::vec::Vec<Aminoacid>
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


fn make_aa_table() -> std::vec::Vec<Aminoacid> {
    let aa_holder: std::vec::Vec<Aminoacid> = vec![
        Aminoacid::from("a", &["gcu", "gcc", "gca", "gcg"]),
        Aminoacid::from("c", &["ugu", "ugc"]),
        Aminoacid::from("d", &["gau", "gac"]),
        Aminoacid::from("e", &["gaa", "gag"]),
        Aminoacid::from("f", &["uuu", "uuc"]),
        Aminoacid::from("g", &["ggu", "ggc", "gga", "ggg"]),
        Aminoacid::from("h", &["cau", "cac"]),
        Aminoacid::from("i", &["auu", "auc", "aua"]),
        Aminoacid::from("k", &["aaa", "aag"]),
        Aminoacid::from("l", &["uua", "uug", "cuu", "cuc", "cua", "cug"]),
        Aminoacid::from("m", &["aug"]),
        Aminoacid::from("n", &["ccu", "ccc", "cca", "ccg"]),
        Aminoacid::from("p", &["caa", "cag"]),
        Aminoacid::from("r", &["cgu", "cgc", "cga", "cgg", "aga", "agg"]),
        Aminoacid::from("s", &["ucu", "ucc", "uca", "ucg", "agu", "agc"]),
        Aminoacid::from("v", &["guu", "guc", "gua", "gug"]),
        Aminoacid::from("w", &["ugg"]),
        Aminoacid::from("y", &["uau", "uac"]),
        Aminoacid::from("*", &["uaa", "uag", "uga"])
    ];
    aa_holder
}

fn make_aa_hash_table() -> HashMap<String, String> {
	HashMap::from([
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
	])
}

pub fn dna2aa(data: fasta::Fasta, uppercase: bool) -> Vec<String> {
	let rna_sequence_spl: Vec<String> = data.sequence.chars().map(|c| c.to_string()).collect();
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
        true => aa_seq.to_uppercase().split_inclusive('*').map(|s| s.to_string()).collect(),
        false => aa_seq.split_inclusive('*').map(|s| s.to_string()).collect()
    };
    result
}

pub fn dna2aa_str(data: fasta::Fasta, uppercase: bool) -> String {
	let rna_sequence_spl: Vec<String> = data.sequence.chars().map(|c| c.to_string()).collect();
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
        true => aa_seq.to_uppercase().split_inclusive('*').map(|s| s.to_string()).collect(),
        false => aa_seq.split_inclusive('*').map(|s| s.to_string()).collect()
    };
    result.join("\n")
}

#[cfg(test)]
mod tests {
	use crate::{make_aa_hash_table, Aminoacid, dna2aa, dna2aa_str};

	#[test]
	fn test_struct() {
		let methionine: Aminoacid = Aminoacid::from("m", &["aug"]);
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
