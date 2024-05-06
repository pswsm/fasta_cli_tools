use anyhow::anyhow;

use crate::ctxs::{
    fasta::domain::fasta::C_RNA_BASES,
    shared::domain::{Sequence, SequenceObject},
};

#[derive(Debug, PartialEq, Eq)]
struct Rna {
    chain: SequenceObject<char>,
}

impl TryFrom<String> for Rna {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Rna, anyhow::Error> {
        Self::ensure_dna(&value)?;
        Ok(Rna {
            chain: value.chars().collect::<Vec<char>>(),
        })
    }
}

impl Sequence<char> for Rna {
    fn sequence_type(&self) -> crate::ctxs::shared::domain::SequenceType {
        crate::ctxs::shared::domain::SequenceType::Rna
    }

    fn get_chain(&self) -> SequenceObject<char> {
        self.chain.clone()
    }
}

impl Rna {
    fn ensure_dna(value: &str) -> Result<bool, anyhow::Error> {
        // TODO: better error ensure
        for base in value.chars() {
            if !(C_RNA_BASES.contains(&base)) {
                return Err(anyhow!(format!(
                    "Expected {:?}, found {}",
                    C_RNA_BASES, base
                )));
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::rna::domain::Rna;

    #[test]
    fn try_from_string() {
        let dna_one: Rna = Rna {
            chain: vec!['a', 'u', 'c', 'g'],
        };
        let dna_two: Rna = Rna::try_from("aucg".to_string()).unwrap();
        assert_eq!(dna_one, dna_two)
    }

    #[test]
    #[should_panic]
    fn try_from_invalid_string() {
        match Rna::try_from("aucd".to_string()) {
            Err(_) => panic!(""),
            _ => unreachable!(),
        };
    }
}
