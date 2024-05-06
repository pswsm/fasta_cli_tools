use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;

use crate::ctxs::{fasta::domain::fasta::Fasta, protein::domain::Protein};

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

impl CommonWriteFormat {
    pub(crate) fn save(&self, file_name: &PathBuf) -> Result<()> {
        let mut output_file: File = File::create(file_name)?;
        if self.header.is_some() {
            output_file.write_all(self.header.as_ref().unwrap().as_bytes())?;
        }
        output_file.write_all(self.chain.as_bytes())?;
        Ok(())
    }
}
