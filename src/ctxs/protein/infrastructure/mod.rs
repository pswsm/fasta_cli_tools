use std::path::PathBuf;

use crate::ctxs::shared::{error, infrastructure::CommonWriteFormat};

use super::domain::Protein;

impl Protein {
    pub fn save(self, file: &PathBuf) -> Result<(), error::Generic> {
        CommonWriteFormat::from(self).save(file)
    }
}
