use std::{
    path::Path,
    fs::File,
    io::{
        prelude::Read,
        BufReader
    },
    collections::BTreeMap,
};

use crate::fasta_ops::fasta::Fasta;

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

    let fasta_as_string: String = format!("{}\n{}", header, sequence);
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

pub fn analize(file: &Path) -> std::io::Result<String> {
    let fasta: Fasta = match cat(&file) {
        Ok(seq) => seq,
        Err(e)  => panic!("Can't read file. Error: {}", e)
    };
    let tot_chars: usize = fasta.sequence.chars().count();
    let chars: Vec<char> = fasta.sequence.chars().collect();
    let mut c_count: usize = 0;
    let mut g_count: usize = 0;
    let mut a_count: usize = 0;
    let mut t_count: usize = 0;
    for chr in chars {
        match chr {
            'a' => a_count = a_count + 1,
            't' => t_count = t_count + 1,
            'c' => c_count = c_count + 1,
            'g' => g_count = g_count + 1,
            _ => panic!("Non-dna related character detected.")
        };
    };
    let gc_pct: f32 = ((g_count + c_count) as f32 * 100_f32) / tot_chars as f32;
    let at_pct: f32 = ((a_count + t_count) as f32 * 100_f32) / tot_chars as f32;

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

#[allow(dead_code)]
fn search_hairpin(sequence: String) -> Result<String, String> {
   unimplemented!();
}
