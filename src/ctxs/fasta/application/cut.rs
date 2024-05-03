use std::path::PathBuf;

use anyhow::Result;

use crate::ctxs::fasta::domain::fasta::Fasta;

use super::view;

/// Reads a file, parses it as `Fasta` and cuts the sequence from given indices. This function will
/// write the resulting cut sequence to a given file.
pub fn cut(input_file: PathBuf, output_file: PathBuf, start: usize, end: usize) -> Result<String> {
    let fasta: Fasta = match view::cat_f(&input_file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file!. Error {}", e),
    };
    let cut_fasta: Fasta = fasta.cut(start, end);
    cut_fasta.save(&output_file)?;
    let result: String = format!(
        "Cut from {} to {}. Read {}. Write {}",
        start,
        end,
        input_file.display(),
        output_file.display()
    );
    Ok(result)
}
