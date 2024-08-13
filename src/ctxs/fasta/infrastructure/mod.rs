use std::path::PathBuf;

use crate::ctxs::shared::{error::Generic, infrastructure::CommonWriteFormat};

use super::domain::fasta::Fasta;

impl Fasta {
    pub fn save(self, file: &PathBuf) -> Result<(), Generic> {
        CommonWriteFormat::from(self).save(file)
    }
}
