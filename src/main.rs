mod fastaOps;
use fastaOps::{edit, view, make};

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "fasta_cli_toolkit",
            about = "A CLI toolkit to manipulate fasta files",
            rename_all = "kebab-case")]
struct Args {
    #[structopt(subcommand)]
    cmdline: Command
}

#[derive(StructOpt)]
enum Command {
    Print(CatOptions),
    Cut(CutOptions),
    Generate(GenerateOptions),
    Format(FormatOptions)
}

#[derive(StructOpt)]
#[structopt(name = "print file options",
            about = "Reads file, prints its content",
            rename_all = "kebab-case")]
struct CatOptions {
    #[structopt(help = "The file to read")]
    file: PathBuf,
}

#[derive(StructOpt)]
#[structopt(name = "cutting options",
            about = "Cuts nucleotides from..to range",
            rename_all = "kebab-case")]
struct CutOptions {
    #[structopt(help = "Position to start cutting")]
    from: usize,
    
    #[structopt(help = "Position to start cutting")]
    to: usize,

    #[structopt(help = "File to read")] 
    input_file_name: PathBuf,

    #[structopt(help = "File to write")]
    output_file_name: PathBuf,    
}

#[derive(StructOpt)]
#[structopt(name = "generation options",
            about = "Generates a fasta file long n lines",
            rename_all = "kebab-case")]
struct GenerateOptions {
    #[structopt(help = "Number of lines to generate. Each line has 60 bases")] 
    length: u32,

    #[structopt(help = "File to write to")]
    output_file: PathBuf,
}

#[derive(StructOpt)]
#[structopt(name = "format otions",
            about = "Formats a fasta file",
            rename_all = "kebab-case")]
struct FormatOptions {
    #[structopt(help = "File to format")]
    file: PathBuf,

    #[structopt(short, long = "upper", help = "Format to uppercase")]
    uppercase: bool,

    #[structopt(short, long = "output", help = "File to write formatted fasta. Optional")]
    output_file: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    let result = match args.cmdline {
        Command::Cut(args)      => edit::cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or(String::from("Could not cut")),
        Command::Generate(args) => make::generate(args.length, args.output_file).unwrap_or(String::from("Could not generate")),
        Command::Print(args)    => view::cat_as_string(&args.file).unwrap_or(String::from("File not found")),
        Command::Format(args)   => edit::format(args.file, args.uppercase, args.output_file).unwrap_or(String::from("Could not format")),
    };

    println!("{}", result);
}
