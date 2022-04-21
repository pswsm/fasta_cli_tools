use std::fs::File;
use std::io::prelude::Write;
use std::path::PathBuf;
use rand::seq::SliceRandom;

pub fn generate(lines: u32, file: PathBuf) -> std::io::Result<String> {
    let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let header: String = format!(">randomly generated sequence of {} lines\n", lines);
    let mut sequence: String = String::new();

    for _line in 1..=lines {
        for _base in 1..=60 {
            sequence.push_str(&select_rnd_str(&atcg));
        }
        sequence.push_str("\n");
    }
    let mut output_file = File::create(&file)?;
    output_file.write(header.as_bytes())?;
    output_file.write(sequence.as_bytes())?;

    let result: String = format!("Generated file {} with {} bases", file.display(), lines * 60);

    Ok(result)
}

fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String = String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}
