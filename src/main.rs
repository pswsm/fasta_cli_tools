use std::path::PathBuf;
use structopt::StructOpt;

mod fasta;

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
        Command::Cut(args) => fasta::edit::cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or(String::from("Could not cut")),
        Command::Generate(args) => fasta::make::generate(args.length, args.output_file).unwrap_or(String::from("Could not generate")),
        Command::Print(args) => fasta::view::cat(&args.file).unwrap_or(String::from("File not found. Check the if file exists.")),
    };

    println!("{}", result);
}
