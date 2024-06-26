use std::path::PathBuf;

use crate::shared::infrastructure::CommonWriteFormat;

use super::domain::protein::Protein;

impl Protein {
    pub fn save(self, file: &PathBuf) -> Result<(), anyhow::Error> {
        CommonWriteFormat::from(self).save(file)
    }
}
