use std::path::PathBuf;

use crate::ctxs::shared::infrastructure::CommonWriteFormat;

use super::domain::fasta::Fasta;

impl Fasta {
    pub fn save(self, file: &PathBuf) -> Result<(), anyhow::Error> {
        CommonWriteFormat::from(self).save(file)
    }
}
