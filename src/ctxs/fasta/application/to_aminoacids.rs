use std::path::PathBuf;

use crate::ctxs::{fasta::domain::fasta::Fasta, protein::domain::protein::Protein};

pub fn to_aminoacids(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let aas: Protein = Protein::from(fasta);
    if let Some(file) = ofile {
        aas.save(&file)?
    }
    Ok("".to_string())
}
