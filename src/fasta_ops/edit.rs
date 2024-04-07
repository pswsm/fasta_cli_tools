//! Fasta editing utilities
use crate::fasta::{self, FastaHeader};
use crate::fasta_ops::view;
use crate::infrastructure::{write_chain_to_file, CommonWriteFormat};
use anyhow::Result;
use fasta::Fasta;
use std::path::PathBuf;
use textwrap::fill;

/// Reads a file, parses it as `Fasta` and cuts the sequence from given indices. This function will
/// write the resulting cut sequence to a given file.
pub fn cutting(
    input_file: PathBuf,
    output_file: PathBuf,
    start: usize,
    end: usize,
) -> Result<String> {
    let og_fasta: Fasta = match view::cat_f(&input_file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file!. Error {}", e),
    };

    let original_sequence: String = og_fasta.sequence.to_string();
    let original_header: FastaHeader = og_fasta.header;

    let sequence_copy: String = original_sequence.replace('\n', "");

    let cut_sequence: String = match sequence_copy.get(start - 1..end) {
        Some(seq) => seq.to_string(),
        None => panic!("Out of range"),
    };

    let new_sequence: String = format_str(cut_sequence).unwrap_or(sequence_copy);
    let new_header: String = format!(
        "Original Header {{{}}}. Original file: {}. Range: {} to {}",
        original_header,
        input_file.display(),
        start,
        end
    );
    let new_fasta = Fasta::from((new_header, new_sequence));
    write_chain_to_file(&output_file, CommonWriteFormat::from(new_fasta));

    let result: String = format!(
        "Cut from {} to {}. Read {}. Write {}",
        start,
        end,
        input_file.display(),
        output_file.display()
    );
    Ok(result)
}

/// Formats a given `String` to standard fasta line length (60 Characters).
pub fn format_str(sequence: String) -> std::result::Result<String, String> {
    let strip_seq: String = sequence.replace('\n', "");
    let result: String = fill(&strip_seq, 60);
    Ok(result)
}

/// Formats a .fasta file, represented with the `Fasta` struct.
pub fn format(file: PathBuf, is_upper: bool, out_file: PathBuf) -> std::io::Result<String> {
    let fasta: Fasta = match view::cat_f(&file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file. Error {}", e),
    };
    let result: String = String::from("Format OK!");
    let strip_seq: String = fasta.sequence.to_string().replace('\n', "");
    let seq: String = fill(&strip_seq, 60);
    let fmt_fasta: Fasta = Fasta::from((fasta.header.to_string(), seq));
    //let mut output_file = File::create(out_file)?;
    if is_upper {
        let final_fasta: Fasta = Fasta::from((
            fmt_fasta.header.to_string(),
            fmt_fasta.sequence.to_string().to_uppercase(),
        ));
        write_chain_to_file(&out_file, CommonWriteFormat::from(final_fasta));
    } else {
        let final_fasta: Fasta = fmt_fasta;
        write_chain_to_file(&out_file, CommonWriteFormat::from(final_fasta));
    };
    Ok(result)
}
