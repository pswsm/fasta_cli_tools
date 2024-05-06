use std::fmt::{self, Display};

use crate::ctxs::{
    fasta::domain::fasta::Fasta,
    shared::domain::{Sequence, SequenceObject},
};

use super::aminoacid::domain::aminoacid::Aminoacid;

/// Struct representing a protein: a chain of aminoacids
pub struct Protein {
    pub chain: SequenceObject<Aminoacid>,
}

/// From a vector of [crate::Aminoacid] create a new Protein.
///
/// For example:
/// ```
/// let aminoacids: Vec<Aminoacid> = vec![Aminoacid::from(('m', vec![Codon::from_chars('a', 'u', 'g')])), Aminoacid::from('m', vec![Codon::from_chars('a', 'u', 'g')])];
/// let proteins: Protein = Protein::from(aminoacids);
/// ```
impl From<Vec<Aminoacid>> for Protein {
    fn from(aminoacids: std::vec::Vec<Aminoacid>) -> Self {
        Protein { chain: aminoacids }
    }
}

impl From<Fasta> for Protein {
    fn from(value: Fasta) -> Self {
        let rna_sequence_spl: Vec<char> = value.sequence.get_chars().collect();
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
}

impl Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.chain.iter().map(|aa| aa.aminoacid).collect::<String>()
        )
    }
}

impl Sequence<Aminoacid> for Protein {
    fn sequence_type(&self) -> crate::ctxs::shared::domain::SequenceType {
        crate::ctxs::shared::domain::SequenceType::Protein
    }
    fn get_chain(&self) -> crate::ctxs::shared::domain::SequenceObject<Aminoacid> {
        self.chain.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::{
        fasta::domain::fasta::Fasta,
        protein::{aminoacid::domain::aminoacid::Aminoacid, domain::Protein},
    };

    #[test]
    fn protein_from_aminoacids() {
        let aminoacids: std::vec::Vec<Aminoacid> = vec![
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
            Aminoacid::from(('m', vec![['a', 'u', 'g']])),
        ];
        let proteins: Protein = Protein::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }

    #[test]
    fn protein_from_fasta() {
        {
            let ff = Fasta::from(("", "augaggcgauga"));
            let aa_sequence: Protein = Protein::from(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*".to_string())
        }
        {
            let ff = Fasta::from(("", "augaggcgauga"));
            let aa_sequence: Protein = Protein::from(ff);
            assert_eq!(aa_sequence.to_string(), "mrr*".to_string())
        }
    }
}
