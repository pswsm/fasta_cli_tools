//! Fasta file reading using `Fasta` struct.

use std::{
    collections::BTreeMap,
    fs::File,
    io,
    io::{prelude::Read, BufReader},
    path::Path,
};

use fasta::Fasta;

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

/// Fake `Fasta` parser. Returns the contents of the file
pub fn cat(file: &Path) -> io::Result<String> {
    let contents: String = read2str!(file);
    let reader_lines: std::str::Lines = contents.lines();
    let header: String = reader_lines.into_iter().filter(|line| line.starts_with(">")).collect();
    let sequence: String = reader_lines.into_iter().filter(|line| !(line.starts_with(">"))).collect();
    let fasta: Fasta = Fasta::from(&[header, sequence]);
    Ok(fasta.to_string())
}


/// Parses a file to `Fasta` struct, and returns it.
pub fn cat_f(file: &Path) -> Result<Fasta, io::Error> {
    let contents: String = read2str!(file);
    let reader_lines: std::str::Lines = contents.lines();
    let header: String = reader_lines.into_iter().filter(|line| line.starts_with(">")).collect();
    let sequence: String = reader_lines.into_iter().filter(|line| !(line.starts_with(">"))).collect();

    let fasta: Fasta = Fasta::from(&[header, sequence]);

    Ok(fasta)

}

/// Analizes the contents of a DNA or RNA sequence.
pub fn analize(file: &Path) -> Result<String, io::Error> {
    let fasta: Fasta = match cat_f(&file) {
        Ok(seq) => seq,
        Err(e) => panic!("Can't read file. Error: {}", e),
    };
    let tot_chars: usize = fasta.sequence.chars().count();
    let chars: Vec<char> = fasta.sequence.chars().collect();
    let c_count: usize = fasta.sequence.chars().filter(|c| c.to_owned() == 'c').count();
    let g_count: usize = fasta.sequence.chars().filter(|c| c.to_owned() == 'g').count();
    let a_count: usize = fasta.sequence.chars().filter(|c| c.to_owned() == 'a').count();
    let t_count: usize = fasta.sequence.chars().filter(|c| (c.to_owned() == 't' || c.to_owned() == 'u') ).count();
    let gc_pct: f64 = ((g_count + c_count) as f64 * 100_f64) / tot_chars as f64;
    let at_pct: f64 = ((a_count + t_count) as f64 * 100_f64) / tot_chars as f64;

    let data: BTreeMap<String, String> = {
        let mut hm: BTreeMap<String, String> = BTreeMap::new();
        hm.insert("Nucleotides".to_string(), tot_chars.to_string());
        hm.insert("AT Count".to_string(), (a_count + t_count).to_string());
        hm.insert("AT Percent".to_string(), at_pct.to_string());
        hm.insert("GC Count".to_string(), (c_count + g_count).to_string());
        hm.insert("GC Percent".to_string(), gc_pct.to_string());
        hm
    };

    let result: String = data.iter().map(|(k, v)| format!("{}:\t{}\n", k, v)).collect();
    Ok(result)
}
