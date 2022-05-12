use crate::fasta_ops::{
    fasta::Fasta,
    view
};

use std::path::PathBuf;
use std::io::prelude::Write;
use std::fs::File;
use textwrap::fill;

pub fn cutting(input_file: PathBuf, output_file: PathBuf, start: usize, end: usize) -> std::io::Result<String> {
    let og_fasta: Fasta = match view::cat(&input_file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file!. Error {}", e)
    };

    let original_sequence: String = og_fasta.sequence;
    let original_header: String = og_fasta.header;

    let sequence_copy: String = original_sequence.replace("\n", "");

    let cut_sequence: String = match sequence_copy.get(start-1..end) {
        Some(seq) => seq.to_string(),
        None => panic!("Out of range")
    };

    let new_sequence: String = match format_str(cut_sequence) {
        Ok(seq) => seq,
        Err(e) => panic!("Cannot format. {e}")
    };

    let new_header: String = format!(">Original Header {{{}}}. Original file: {}. Range: {} to {}\n", original_header, input_file.display(), start, end);
    let mut output_f = File::create(&output_file)?;
    output_f.write(new_header.as_bytes())?;
    output_f.write(new_sequence.as_bytes())?;
    
    let result: String = format!("Cut from {} to {}. Read {}. Write {}", start, end, input_file.display(), output_file.display());
    Ok(result)
}

pub fn format_str(sequence: String) -> std::result::Result<String, String> {
    let result: String = fill(&sequence, 60); 
    Ok(result)
}


pub fn format(file: PathBuf, is_upper: bool, out_file: PathBuf) -> std::io::Result<String> {
    let fasta: Fasta = match view::cat(&file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file. Error {}", e),
    };
    let result: String = String::from("Format OK!");
    let sequence: String = fasta.sequence;
    let seq: String = fill(&sequence, 60); 
    let mut output_file = File::create(out_file)?;
    if is_upper {
        output_file.write(seq.to_uppercase().as_bytes())?;
        return Ok(result.to_uppercase())
    } else {
        output_file.write(seq.as_bytes())?;
    };
    Ok(result)
}
