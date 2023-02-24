//! Aminoacid and protein operations
use crate::fasta;
use crate::structs::{Aminoacid, ProteinChain};

/// Creates a vector holding all possible aminoacids.
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
        Aminoacid::from(vec!["*", "uaa", "uag", "uga"]),
    ];
    aa_holder
}

/// Returns the `Aminoacid` prot is from.
pub fn aa_compare(prot: &str) -> Aminoacid {
    let aa_table: std::vec::Vec<Aminoacid> = make_aa_table();
    let matched_prot: std::vec::Vec<Aminoacid> =
        aa_table.into_iter().filter(|aa| aa.aa == prot).collect();
    matched_prot[0].clone()
}

/// Returns the aminoacid the codon compiles to.
pub fn codon_compare(codon: &str) -> Aminoacid {
    let aa_table: std::vec::Vec<Aminoacid> = make_aa_table();
    let matched_triplets: std::vec::Vec<Aminoacid> = aa_table
        .into_iter()
        .filter(|aa| aa.codons.iter().any(|c| c == codon))
        .collect();
    matched_triplets[0].clone()
}

/// Translates a given fasta struct and returns a vector of strings, each string being a protein
/// letter.
pub fn dna2aa(data: fasta::Fasta) -> ProteinChain {
    let rna_sequence_spl: Vec<String> = data.sequence.chars().map(|c| c.to_string()).collect();
    let aa_seq = {
        let mut aa_seq_tmp: Vec<Aminoacid> = Vec::new();
        for gidx in (0..(rna_sequence_spl.len())).step_by(3) {
            let group: String = vec![
                rna_sequence_spl[gidx].clone(),
                rna_sequence_spl[gidx + 1].clone(),
                rna_sequence_spl[gidx + 2].clone(),
            ]
            .join("");
            aa_seq_tmp.push(codon_compare(&group));
        }
        aa_seq_tmp
    };
    ProteinChain::from(aa_seq)
}

/// Same as [dna2aa] but returns a String.
pub fn dna2aa_str(data: fasta::Fasta, uppercase: bool) -> String {
    let rna_sequence_spl: Vec<String> = data.sequence.chars().map(|c| c.to_string()).collect();
    let aa_seq: Vec<Aminoacid> = {
        let mut aa_seq_tmp: Vec<Aminoacid> = Vec::new();
        for grop_index in (0..(rna_sequence_spl.len())).step_by(3) {
            let group: String = vec![
                rna_sequence_spl[grop_index].clone(),
                rna_sequence_spl[grop_index + 1].clone(),
                rna_sequence_spl[grop_index + 2].clone(),
            ]
            .join("");
            aa_seq_tmp.push(codon_compare(&group));
        }
        aa_seq_tmp
    };
    match uppercase {
        true => ProteinChain::from(aa_seq).to_string().to_uppercase(),
        false => ProteinChain::from(aa_seq).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::dna2aa::{aa_compare, codon_compare, dna2aa, dna2aa_str};
    use crate::fasta;
    use crate::structs::{Aminoacid, ProteinChain};
    #[test]
    fn test_dna2aa() {
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgauga".to_string(),
            };
            let aa_sequence: ProteinChain = dna2aa(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*".to_string())
        }
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgaugaaugaggcgauga".to_string(),
            };
            let aa_sequence: ProteinChain = dna2aa(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*mrr*".to_string())
        }
    }

    #[test]
    fn test_dna2aa_str() {
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgauga".to_string(),
            };
            let aa_sequence: String = dna2aa_str(ff, true);
            assert_eq!(aa_sequence, "MRR*".to_string())
        }
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgauga".to_string(),
            };
            let aa_sequence: String = dna2aa_str(ff, false);
            assert_eq!(aa_sequence, "mrr*".to_string())
        }
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgaugaaugaggcgauga".to_string(),
            };
            let aa_sequence: String = dna2aa_str(ff, false);
            assert_eq!(aa_sequence, "mrr*\nmrr*".to_string())
        }
    }

    #[test]
    fn compare_aa() {
        let matched_aa: Aminoacid = aa_compare("m");
        let base_aa: Aminoacid = Aminoacid::from(vec!["m", "aug"]);
        assert_eq!(matched_aa.aa, base_aa.aa);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }

    #[test]
    fn compare_codons() {
        let matched_aa: Aminoacid = codon_compare("aug");
        let expected_aa: Aminoacid = Aminoacid::from(vec!["m", "aug"]);
        assert_eq!(matched_aa.aa, expected_aa.aa);
        assert_eq!(matched_aa.codons, expected_aa.codons);
    }
}
