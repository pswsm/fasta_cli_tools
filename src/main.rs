use std::path::{PathBuf, Path};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use structopt::StructOpt;
use std::vec;
use rand::seq::SliceRandom;

#[derive(StructOpt)]
#[structopt(name = "CLI fasta toolkit",
            about = "A CLI toolkit to generate, edit and see fasta files",
            rename_all = "kebab-case")]
struct Args {
    #[structopt(subcommand)]
    cmdline: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    Print(CatOptions),
    Cut(CutOptions),
    Generate(GenerateOptions)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "print file options",
            about = "Reads file, prints its content",
            rename_all = "kebab-case")]
struct CatOptions {
    #[structopt(help = "The file to read")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cutting options",
            about = "Cuts nucleotides from..to range",
            rename_all = "kebab-case")]
struct CutOptions {
    #[structopt(short, long = "output", help = "File to write", parse(from_os_str))]
    output_file_name: PathBuf,
    
    #[structopt(short, long = "input", help = "File to read")] 
    input_file_name: PathBuf,

    #[structopt(help = "Position to start cutting")]
    from: usize,
    
    #[structopt(help = "Position to start cutting")]
    to: usize
}

#[derive(Debug, StructOpt)]
#[structopt(name = "generation options",
            about = "Generates a fasta file long n bases",
            rename_all = "kebab-case")]
struct GenerateOptions {

    #[structopt(help = "File to write to")]
    output_file: PathBuf,

    #[structopt(short = "n", long = "lines", help = "Number of lines to generate. Each line has 60 bases")] 
    length: u32,
}

fn main() {

    let args = Args::from_args();

    let result = match args.cmdline {
        Command::Cut(args) => cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or(String::from("Could not cut")),
        Command::Generate(args) => generate(args.length, args.output_file).unwrap_or(String::from("Could not generate")),
        Command::Print(args) => cat(args.file).unwrap_or(String::from("File not found. Check the if file exists.")),
    };

    println!("{}", result);
}

fn cutting(input_file: PathBuf, output_file: PathBuf, start: usize, end: usize) -> std::io::Result<String> {
    let text: String = match cat(&input_file) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read file!. Error {}", e)
    };
    let mut text_lines = text.lines();
    let mut sequence: String = String::new();
    text_lines.next();
    text_lines.next();
    for line in text_lines {
        sequence.push_str(line);
    };

    let seq_copy: String = String::from(&sequence);
    for char in seq_copy.char_indices() {
        if char.1.to_string() == "\n" {
            sequence.remove(char.0);
        }
    };

    let header: String = format!("> Original file: {}. Range {} to {}", input_file.display().to_string(), start, end);
    let mut cut_fasta: String = sequence.get(start..end).unwrap_or("Out of range.").to_string();

    for indice in (59..cut_fasta.len()).step_by(60) {
        cut_fasta.insert_str(indice, "\n");
    };

    let result: String = format!("{}\n{}", header, cut_fasta);

    let mut output_f = File::create(&output_file)?;
    output_f.write(result.as_bytes())?;

    Ok(result)
}

fn cat<PB: AsRef<Path>>(input_file: PB) -> std::io::Result<String> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn generate(lines: u32, file: PathBuf) -> std::io::Result<String> {
    let mut output_file = File::create(&file)?;
    let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let header: String = format!(">randomly generated sequence of {} lines\n", lines);
    let mut sequence: String = String::new();

    output_file.write(header.as_bytes())?;
    output_file.write(b"\n")?;

    for _line in 1..=lines {
        for _base in 1..=60 {
            sequence.push_str(&select_rnd_str(&atcg));
        }
        sequence.push_str("\n");
    }

    output_file.write(sequence.as_bytes())?;

    let result: String = format!("Generated file {} with {} bases", file.display(), lines * 60);

    Ok(result)
}

fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String = String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}
