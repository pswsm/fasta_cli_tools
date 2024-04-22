//! Aminoacid and protein operations
use crate::fasta;
use crate::structs::{Aminoacid, Codon, Protein};

/// Fasta to Protein translation
pub fn fasta_to_protein(data: fasta::Fasta) -> Protein {
    let rna_sequence_spl: Vec<char> = data.sequence.get_chars().collect();
    let aa_seq = {
        let mut aa_seq_tmp: Vec<Aminoacid> = Vec::new();
        for gidx in (0..(rna_sequence_spl.len())).step_by(3) {
            let group: [char; 3] = [
                rna_sequence_spl[gidx],
                rna_sequence_spl[gidx + 1],
                rna_sequence_spl[gidx + 2],
            ];
            aa_seq_tmp.push(Aminoacid::get_aminoacd_from_codon(Codon::from_chars(group)));
        }
        aa_seq_tmp
    };
    Protein::from(aa_seq)
}

#[cfg(test)]
mod tests {
    use crate::dna2aa::fasta_to_protein;
    use crate::fasta::Fasta;
    use crate::structs::{Aminoacid, Codon, Protein};
    #[test]
    fn test_dna2aa() {
        {
            let ff = Fasta::from(("", "augaggcgauga"));
            let aa_sequence: Protein = fasta_to_protein(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*".to_string())
        }
        {
            let ff = Fasta::from(("", "augaggcgauga"));
            let aa_sequence: Protein = fasta_to_protein(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*mrr*".to_string())
        }
    }

    #[test]
    fn get_from_codon() {
        let matched_aa: Aminoacid =
            Aminoacid::get_aminoacd_from_codon(Codon::from_chars(['a', 'u', 'g']));
        let base_aa: Aminoacid = Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])]));
        assert_eq!(matched_aa.aminoacid, base_aa.aminoacid);
        assert_eq!(matched_aa.codons, base_aa.codons);
    }
}
