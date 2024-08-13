use std::path::PathBuf;

use crate::ctxs::{fasta::domain::fasta::Fasta, protein::domain::Protein, shared::error::Generic};

pub fn to_aminoacids(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, Generic> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let aas: Protein = Protein::from(fasta);
    if let Some(file) = ofile {
        aas.save(&file)?
    }
    Ok("".to_string())
}
