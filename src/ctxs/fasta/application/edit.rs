//! Fasta editing utilities
use anyhow::Result;
use std::path::PathBuf;
use textwrap::fill;

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

/// Formats a .fasta file, represented with the `Fasta` struct.
pub fn format(file: PathBuf, is_upper: bool, out_file: PathBuf) -> Result<String> {
    let fasta: Fasta = match view::cat_f(&file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file. Error {}", e),
    };
    let result: String = String::from("Format OK!");
    let strip_seq: String = fasta.sequence.to_string().replace('\n', "");
    let seq: String = fill(&strip_seq, 60);
    let fmt_fasta: Fasta = Fasta::from((fasta.header.to_string(), seq));
    let final_fasta: Fasta = match is_upper {
        true => fmt_fasta.uppercase(),
        false => fmt_fasta,
    };
    final_fasta.save(&out_file)?;
    Ok(result)
}
