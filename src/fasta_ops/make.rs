use std::fs::File;
use std::io::prelude::Write;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use crate::fasta_ops::edit::format_str;

pub fn generate(bases: u32, file: PathBuf) -> std::io::Result<String> {
    let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let header: String = format!(">randomly generated sequence of {} bases\n", bases);
    let mut sequence: String = String::new();

    for _base in 1..=bases {
        sequence.push_str(&select_rnd_str(&atcg));
    }

    let fmt_sequence: String = match format_str(sequence) {
        Ok(seq) => seq,
        Err(e) => panic!("Could not format. Error {}", e)
    };
    let mut output_file = File::create(&file)?;
    output_file.write(header.as_bytes())?;
    output_file.write(fmt_sequence.as_bytes())?;

    let result: String = format!("Generated file {} with {} bases", file.display(), bases);

    Ok(result)
}

fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String = String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}
