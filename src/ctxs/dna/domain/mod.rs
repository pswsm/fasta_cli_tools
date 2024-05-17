use anyhow::anyhow;

use crate::ctxs::{
    fasta::{application::generate::c_generate_bases, domain::fasta::C_DNA_BASES},
    shared::domain::{Sequence, SequenceValueObject},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    chain: SequenceValueObject<char>,
}

impl TryFrom<String> for Dna {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Dna, anyhow::Error> {
        Self::ensure_dna(&value)?;
        Ok(Dna {
            chain: value.chars().collect::<Vec<char>>(),
        })
    }
}

impl Sequence<char> for Dna {
    fn sequence_type(&self) -> crate::ctxs::shared::domain::SequenceType {
        crate::ctxs::shared::domain::SequenceType::Dna
    }

    fn get_chain(&self) -> SequenceValueObject<char> {
        self.chain.clone()
    }

    fn generate() -> Self {
        Dna {
            chain: c_generate_bases(300, C_DNA_BASES).unwrap(),
        }
    }
}

impl Dna {
    fn ensure_dna(value: &str) -> Result<bool, anyhow::Error> {
        // TODO: better error ensure
        for base in value.chars() {
            if !(C_DNA_BASES.contains(&base)) {
                return Err(anyhow!(format!(
                    "Expected {:?}, found {}",
                    C_DNA_BASES, base
                )));
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::ctxs::{dna::domain::Dna, shared::domain::Sequence};

    #[test]
    fn try_from_string() {
        let dna_one: Dna = Dna {
            chain: vec!['a', 't', 'c', 'g'],
        };
        let dna_two: Dna = Dna::try_from("atcg".to_string()).unwrap();
        assert_eq!(dna_one, dna_two)
    }

    #[test]
    #[should_panic]
    fn try_from_invalid_string() {
        match Dna::try_from("atcd".to_string()) {
            Err(_) => panic!(""),
            _ => unreachable!(),
        };
    }

    #[test]
    fn try_generate() {
        let _: Dna = Dna::generate();
    }
}
