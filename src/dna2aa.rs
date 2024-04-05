//! Aminoacid and protein operations
use crate::fasta;
use crate::structs::{Aminoacid, AminoacidValue, Protein};
use crate::utils::AMINOACID_TABLE;

/// Fetches the selected aminoacid object from the aminoacid table
pub fn fetch_aminoacid(aminoacid_to_fetch: AminoacidValue) -> Aminoacid {
    let aa_table: std::vec::Vec<Aminoacid> = AMINOACID_TABLE;
    let matched_prot: std::vec::Vec<Aminoacid> = aa_table
        .into_iter()
        .filter(|aminoacid| aminoacid.aminoacid == aminoacid_to_fetch)
        .collect();
    matched_prot[0].clone()
}

/// Translates a given fasta struct and returns a protein
pub fn dna2aa(data: fasta::Fasta) -> Protein {
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
            aa_seq_tmp.push(Aminoacid::from(group));
        }
        aa_seq_tmp
    };
    Protein::from(aa_seq)
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
        true => Protein::from(aa_seq).to_string().to_uppercase(),
        false => Protein::from(aa_seq).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::dna2aa::{codon_compare, dna2aa, dna2aa_str, fetch_aminoacid};
    use crate::fasta;
    use crate::structs::{Aminoacid, Protein};
    #[test]
    fn test_dna2aa() {
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgauga".to_string(),
            };
            let aa_sequence: Protein = dna2aa(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*".to_string())
        }
        {
            let ff: fasta::Fasta = fasta::Fasta {
                header: "".to_string(),
                sequence: "augaggcgaugaaugaggcgauga".to_string(),
            };
            let aa_sequence: Protein = dna2aa(ff);
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
        let matched_aa: Aminoacid = fetch_aminoacid('m');
        let base_aa: Aminoacid = Aminoacid::from(('m', vec![['a', 'u', 'g']]));
        assert_eq!(matched_aa.aminoacid, base_aa.aminoacid);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }

    #[test]
    fn compare_codons() {
        let matched_aa: Aminoacid = codon_compare("aug");
        let base_aa: Aminoacid = Aminoacid::from(('m', vec![['a', 'u', 'g']]));
        assert_eq!(matched_aa.aminoacid, expected_aa.aminoacid);
        assert_eq!(matched_aa.codons, expected_aa.codons);
    }
}
