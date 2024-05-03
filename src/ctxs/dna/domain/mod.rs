use anyhow::anyhow;

use crate::ctxs::{
    fasta::domain::fasta::DNA_BASES,
    shared::domain::{Chain, ChainObject},
};

pub struct Dna {
    chain: ChainObject,
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

impl Chain for Dna {
    fn chain_type(&self) -> crate::ctxs::shared::domain::ChainTypes {
        crate::ctxs::shared::domain::ChainTypes::DNA
    }
}

impl Dna {
    fn ensure_dna(value: &str) -> Result<bool, anyhow::Error> {
        match value.split("").all(|c| DNA_BASES.contains(&c)) {
            true => Ok(true),
            false => Err(anyhow!("AAAAA")),
        }
    }

    pub fn get_chain(&self) -> ChainObject {
        self.chain.clone()
    }
}
