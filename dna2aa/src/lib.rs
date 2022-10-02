use std::collections::HashMap;
use crate::structs::{
    aminoacid::Aminoacid,
    proteins::ProteinChain
};
mod structs;

fn make_aa_table() -> std::vec::Vec<Aminoacid> {
    let aa_holder: std::vec::Vec<Aminoacid> = vec![
        Aminoacid::from(vec!["a", "gcu", "gcc", "gca", "gcg"]),
        Aminoacid::from(vec!["c", "ugu", "ugc"]),
        Aminoacid::from(vec!["d", "gau", "gac"]),
        Aminoacid::from(vec!["e", "gaa", "gag"]),
        Aminoacid::from(vec!["f", "uuu", "uuc"]),
        Aminoacid::from(vec!["g", "ggu", "ggc", "gga", "ggg"]),
        Aminoacid::from(vec!["h", "cau", "cac"]),
        Aminoacid::from(vec!["i", "auu", "auc", "aua"]),
        Aminoacid::from(vec!["k", "aaa", "aag"]),
        Aminoacid::from(vec!["l", "uua", "uug", "cuu", "cuc", "cua", "cug"]),
        Aminoacid::from(vec!["m", "aug"]),
        Aminoacid::from(vec!["n", "ccu", "ccc", "cca", "ccg"]),
        Aminoacid::from(vec!["p", "caa", "cag"]),
        Aminoacid::from(vec!["r", "cgu", "cgc", "cga", "cgg", "aga", "agg"]),
        Aminoacid::from(vec!["s", "ucu", "ucc", "uca", "ucg", "agu", "agc"]),
        Aminoacid::from(vec!["v", "guu", "guc", "gua", "gug"]),
        Aminoacid::from(vec!["w", "ugg"]),
        Aminoacid::from(vec!["y", "uau", "uac"]),
        Aminoacid::from(vec!["*", "uaa", "uag", "uga"])
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
	use crate::{make_aa_hash_table, dna2aa, dna2aa_str, structs::{aminoacid::Aminoacid, proteins::ProteinChain}};

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
