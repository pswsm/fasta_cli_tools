use std::path::Path;
use std::fs::File;
use std::io::prelude::Read;
use std::io::BufReader;
use crate::fasta_ops::{fasta::Fasta, edit::format_str};

macro_rules! read2str {
    ($path:ident) => {
        {
            let file: File = File::open($path)?;
            let mut reader: BufReader<File> = BufReader::new(file);
            let mut contents: String = String::new();
            reader.read_to_string(&mut contents)?;
            contents
        }
    };
}

pub fn cat_as_string(file: &Path) -> std::io::Result<String> {
    let contents = read2str!(file);
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

pub fn cat(file: &Path) -> std::io::Result<Fasta> {
    let contents = read2str!(file);
    let reader_lines = contents.lines();

    let mut header: String = String::new();
    let mut sequence: String = String::new();
    for line in reader_lines {
        if line.starts_with(">") {
            header.push_str(line);
        } else if !line.starts_with(">") {
            sequence.push_str(line);
        } else {
            panic!{"This should not happen"};
        };
    };

    let fasta: Fasta = Fasta::from(header, sequence);
    Ok(fasta)
}

pub fn analize(file: &Path) -> std::io::Result<Fasta> {
    let contents = read2str!(file);
    unimplemented!()
}
