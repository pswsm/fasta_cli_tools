pub mod view {
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::Read;
    use std::io::BufReader;

    pub fn cat(input_file: &Path) -> std::io::Result<String> {
        let file = File::open(input_file)?;
        let mut reader = BufReader::new(file);
        let mut contents: String = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub mod edit {
    use crate::fasta::view;

    use std::path::PathBuf;
    use std::io::prelude::Write;
    use std::fs::File;
    use textwrap::fill;

    pub fn cutting(input_file: PathBuf, output_file: PathBuf, start: usize, end: usize) -> std::io::Result<String> {
        let text: String = match view::cat(&input_file) {
            Ok(contents) => contents,
            Err(e) => panic!("Could not read file!. Error {}", e)
        };

        let mut text_lines = text.lines();

        let mut original_header: String = String::new();
        if text.chars().next() == Some('>') {
            let og_header: &str  = match text_lines.next() {
                Some(header) => header,
                None => ">"
            };
            original_header.push_str(og_header);
        };

        let mut sequence: String = String::new();
        // Skips the header and pushes sequence lines to v:sequence
        for line in text_lines {
            sequence.push_str(line);
        };

        let seq_copy: String = String::from(&sequence);
        for char in seq_copy.char_indices() {
            if char.1.to_string() == "\n" {
                sequence.remove(char.0);
            }
        };

        let cut_fasta: String = match sequence.get(start-1..end) {
            Some(seq) => seq.to_string(),
            None => panic!("Out of range")
        };

        let return_fasta: String = match format_str(cut_fasta) {
            Ok(seq) => seq,
            Err(e) => panic!("Cannot format. {e}")
        };

        let header: String = format!(">Original Header {{{}}}. Original file: {}. Range: {} to {}\n", original_header, input_file.display(), start, end);
        let mut output_f = File::create(&output_file)?;
        output_f.write(header.as_bytes())?;
        output_f.write(return_fasta.as_bytes())?;
        
        let result: String = format!("Cut from {} to {}. Read {}. Write {}", start, end, input_file.display(), output_file.display());
        Ok(result)
    }

    fn format_str(sequence: String) -> std::result::Result<String, String> {
        let result: String = fill(&sequence, 60); 
        Ok(result)
    }


    pub fn format(file: PathBuf, is_upper: bool, out_file: Option<PathBuf>) -> std::io::Result<String> {
        let text = match view::cat(&file) {
            Ok(contents) => contents,
            Err(e) => panic!("Could not read file. Error {}", e),
        };

        let mut text_lines = text.lines();
        text_lines.next();
        let sequence: String = String::from(text_lines.next().unwrap());

        let result = fill(&sequence, 60); 

        if out_file != None {
            let mut output_file = File::create(out_file.unwrap())?;
            if is_upper {
                output_file.write(result.to_uppercase().as_bytes())?;
            } else if !is_upper {
                output_file.write(result.as_bytes())?;
            }
        };

        if is_upper {
            return Ok(result.to_uppercase())
        }

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
