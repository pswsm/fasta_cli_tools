use std::path::PathBuf;

use crate::ctxs::shared::infrastructure::CommonWriteFormat;

use super::domain::Protein;

impl Protein {
    pub fn save(self, file: &PathBuf) -> Result<(), anyhow::Error> {
        CommonWriteFormat::from(self).save(file)
    }
}
