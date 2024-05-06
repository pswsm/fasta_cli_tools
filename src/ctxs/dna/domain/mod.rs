use anyhow::anyhow;

use crate::ctxs::{
    fasta::domain::fasta::DNA_BASES,
    shared::domain::{Sequence, SequenceObject},
};

struct Dna {
    chain: SequenceObject,
}

impl TryFrom<String> for Dna {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Dna, anyhow::Error> {
        Self::ensure_dna(&value)?;
        Ok(Dna {
            chain: value
                .split("")
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
        })
    }
}

impl Sequence for Dna {
    fn sequence_type(&self) -> crate::ctxs::shared::domain::SequenceType {
        crate::ctxs::shared::domain::SequenceType::Dna
    }
}

impl Dna {
    fn ensure_dna(value: &str) -> Result<bool, anyhow::Error> {
        match value.split("").all(|c| DNA_BASES.contains(&c)) {
            true => Ok(true),
            false => Err(anyhow!("AAAAA")),
        }
    }

    pub fn get_chain(&self) -> SequenceObject {
        self.chain.clone()
    }
}
