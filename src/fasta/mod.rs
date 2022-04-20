pub struct Fasta {
    header: String,
    sequence: String
}

impl Fasta {
    /* 
    const fn new() -> Fasta {
        Fasta { header: String::new(), sequence: String::new() }
    }
     */

    const fn from(headr: String, seqnc: String) -> Fasta {
        Fasta { header: headr, sequence: seqnc }
    }
}

pub mod view {
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::Read;
    use std::io::BufReader;

    use crate::fasta::{Fasta, edit::format_str};

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
}

pub mod edit {
    use crate::fasta::{view, Fasta};

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


    pub fn format(file: PathBuf, is_upper: bool, out_file: Option<PathBuf>) -> std::io::Result<String> {
        let fasta = match view::cat(&file) {
            Ok(contents) => contents,
            Err(e) => panic!("Could not read file. Error {}", e),
        };

        let sequence: String = fasta.sequence;
        let result = fill(&sequence, 60); 

        if out_file != None {
            let mut output_file = File::create(out_file.unwrap())?;
            if is_upper {
                output_file.write(result.to_uppercase().as_bytes())?;
            } else if !is_upper {
                output_file.write(result.as_bytes())?;
            };
        };

        if is_upper {
            return Ok(result.to_uppercase())
        };

        Ok(result)

    }
}

pub mod make{
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
}
