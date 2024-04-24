// TODO: Move funtionality
use super::{
    aminoacid::domain::aminoacid::Aminoacid, fasta::domain::fasta::Fasta,
    protein::domain::protein::Protein,
};

/// Fasta to Protein translation
pub fn fasta_to_protein(data: Fasta) -> Protein {
    let rna_sequence_spl: Vec<char> = data.sequence.get_chars().collect();
    let aa_seq = {
        let mut aa_seq_tmp: Vec<Aminoacid> = Vec::new();
        for gidx in (0..(rna_sequence_spl.len())).step_by(3) {
            let group: [char; 3] = [
                rna_sequence_spl[gidx],
                rna_sequence_spl[gidx + 1],
                rna_sequence_spl[gidx + 2],
            ];
            aa_seq_tmp.push(Aminoacid::from(group));
        }
        aa_seq_tmp
    };
    Protein::from(aa_seq)
}

#[cfg(test)]
mod tests {
    use crate::ctxs::{
        dna2aa::fasta_to_protein, fasta::domain::fasta::Fasta, protein::domain::protein::Protein,
    };

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
            assert_eq!(aa_sequence.to_string(), "mrr*".to_string())
        }
    }
}
