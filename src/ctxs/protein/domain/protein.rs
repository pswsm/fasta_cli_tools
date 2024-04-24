use std::fmt::{self, Display};

use crate::ctxs::{aminoacid::domain::aminoacid::Aminoacid, fasta::domain::fasta::Fasta};

/// Struct representing a protein: a chain of aminoacids
pub struct Protein {
    /// chain: A Vector holding Aminoacid structs
    pub chain: Vec<Aminoacid>,
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

#[cfg(test)]
mod tests {
    use crate::ctxs::{
        aminoacid::domain::aminoacid::Aminoacid, codon::domain::codon::Codon,
        protein::domain::protein::Protein,
    };

    #[test]
    fn structs_protein() {
        let aminoacids: std::vec::Vec<Aminoacid> = vec![
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
            Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
        ];
        let proteins: Protein = Protein::from(aminoacids);
        assert_eq!(proteins.to_string(), "mmmm".to_string())
    }
}
