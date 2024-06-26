//! Fasta file reading using `Fasta` struct.

use std::{
    collections::BTreeMap,
    fmt::Write,
    fs::File,
    io::{prelude::Read, BufReader},
    path::Path,
};

use crate::ctxs::fasta::domain::fasta::Fasta;

/// Reads files contents and returns them, independently of type.
macro_rules! read2str {
    ($path:ident) => {{
        let file: File = File::open($path)?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut contents: String = String::new();
        reader.read_to_string(&mut contents)?;
        contents
    }};
}

/// A wrapper function around `cat_f()` that returns a string.
pub fn cat(file: &Path) -> anyhow::Result<String, anyhow::Error> {
    /* if let Ok(fasta) = cat_f(file) {
        return Ok(fasta.to_string())
    }
    if let Err(fasta) = cat_f(file) {
        return Err(fasta)
    } else {

    } */
    Ok(cat_f(file)
        .unwrap_or_else(|_| Fasta::from(("Could not open file!", "")))
        .to_string())
}

/// Parses a file to `Fasta` struct, and returns it.
pub fn cat_f(file: &Path) -> Result<Fasta, anyhow::Error> {
    let contents: String = read2str!(file);
    let reader_lines: std::str::Lines = contents.lines();
    let reader_lines_copy: std::str::Lines = reader_lines.clone();
    let header: String = reader_lines
        .into_iter()
        .filter(|line| line.starts_with('>'))
        .map(|line| line.trim_start_matches("> ").to_owned())
        .collect();
    let sequence: String = reader_lines_copy
        .into_iter()
        .filter(|line| !(line.starts_with('>')))
        .collect();
    if sequence.is_empty() {
        return Err(anyhow::anyhow!("Sequence is empty.").context("The file has an empty sequence"));
    }

    let fasta: Fasta = Fasta::from((header, sequence));

    Ok(fasta)
}

/// Analizes the contents of a DNA or RNA sequence.
pub fn analize(file: &Path) -> Result<String, anyhow::Error> {
    let fasta: Fasta = match cat_f(file) {
        Ok(seq) => seq,
        Err(e) => return Err(anyhow::anyhow!("Can't read file. Error: {}", e)),
    };
    let t_chars: usize = fasta.sequence.get_chars().count();
    let c_count: usize = fasta.sequence.get_chars().filter(|&c| c == 'c').count();
    let g_count: usize = fasta.sequence.get_chars().filter(|&c| c == 'g').count();
    let a_count: usize = fasta.sequence.get_chars().filter(|&c| c == 'a').count();
    let t_count: usize = fasta
        .sequence
        .get_chars()
        .filter(|&c| (c == 't' || c == 'u'))
        .count();
    let gc_pct: f64 = ((g_count + c_count) as f64 * 100_f64) / t_chars as f64;
    let at_pct: f64 = ((a_count + t_count) as f64 * 100_f64) / t_chars as f64;

    let data: BTreeMap<String, String> = {
        let mut hm: BTreeMap<String, String> = BTreeMap::new();
        hm.insert("Nucleotides".to_string(), t_chars.to_string());
        hm.insert("AT Count".to_string(), (a_count + t_count).to_string());
        hm.insert("AT Percent".to_string(), at_pct.to_string());
        hm.insert("GC Count".to_string(), (c_count + g_count).to_string());
        hm.insert("GC Percent".to_string(), gc_pct.to_string());
        hm
    };

    let result: String = data
        .into_iter()
        .fold(String::new(), |mut output, (key, value)| {
            let _ = writeln!(output, "{}:\t{}", key, value);
            output
        });

    Ok(result)
}
