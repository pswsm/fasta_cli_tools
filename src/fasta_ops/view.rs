use std::path::Path;
use std::fs::File;
use std::io::prelude::Read;
use std::io::BufReader;

use crate::fasta_ops::{fasta::Fasta, edit::format_str};

pub fn cat_as_string(input_file: &Path) -> std::io::Result<String> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents)?;
    let reader_lines = contents.lines();

    let mut header: String = String::new();
    let mut sequence: String = String::new();
    for line in reader_lines {
        if line.starts_with(">") {
            header.push_str(line);
        } else if !line.starts_with(">") {
            sequence.push_str(line);
        } else {
            panic!{"Yes."};
        };
    };

    let fasta: Fasta = Fasta::from(header, sequence);
    let fasta_sequence: String = match format_str(fasta.sequence) {
        Ok(seq) => seq,
        Err(e) => panic!("Could not format. {}", e)
    }; 
    let fasta_as_string: String = format!("{}\n{}", fasta.header, fasta_sequence);
    Ok(fasta_as_string)
}

pub fn cat(input_file: &Path) -> std::io::Result<Fasta> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents)?;
    let reader_lines = contents.lines();

    let mut header: String = String::new();
    let mut sequence: String = String::new();
    for line in reader_lines {
        if line.starts_with(">") {
            header.push_str(line);
        } else if !line.starts_with(">") {
            sequence.push_str(line);
        } else {
            panic!{"Yes."};
        };
    };

    let fasta: Fasta = Fasta::from(header, sequence);
    Ok(fasta)
}
