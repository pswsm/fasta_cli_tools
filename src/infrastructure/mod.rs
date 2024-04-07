use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;

use crate::{fasta::Fasta, structs::Protein};

pub struct CommonWriteFormat {
    header: Option<String>,
    chain: String,
}

impl From<Fasta> for CommonWriteFormat {
    fn from(value: Fasta) -> Self {
        CommonWriteFormat {
            header: Some(value.header.to_string()),
            chain: value.sequence.to_string(),
        }
    }
}

impl From<Protein> for CommonWriteFormat {
    fn from(value: Protein) -> Self {
        CommonWriteFormat {
            header: None,
            chain: value.to_string(),
        }
    }
}

pub(crate) fn write_chain_to_file(file_name: &PathBuf, chain: CommonWriteFormat) -> Result<()> {
    let mut output_file: File = File::create(file_name)?;
    if chain.header.is_some() {
        output_file.write_all(chain.header.unwrap().as_bytes())?;
    }
    output_file.write_all(chain.chain.as_bytes())?;
    Ok(())
}
