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
    #[structopt(short, long = "output", help = "File to write", parse(from_os_str))]
    output_file_name: PathBuf,
    
    #[structopt(short, long = "input", help = "File to read")] 
    input_file_name: PathBuf,

    #[structopt(help = "Position to start cutting")]
    from: usize,
    
    #[structopt(help = "Position to start cutting")]
    to: usize
}

#[derive(StructOpt)]
#[structopt(name = "generation options",
            about = "Generates a fasta file long n bases",
            rename_all = "kebab-case")]
struct GenerateOptions {
    #[structopt(help = "Number of lines to generate. Each line has 60 bases")] 
    length: u32,

    #[structopt(help = "File to write to")]
    output_file: PathBuf,
}

#[derive(StructOpt)]
#[structopt(name = "fromat otions",
            help = "Formats a fasta file",
            rename_all = "kebab-case")]
struct FormatOptions {
    #[structopt(help = "File to format")]
    file: PathBuf,

    #[structopt(short, long = "upper", help = "If present, format to uppercase")]
    uppercase: bool,

    #[structopt(short, long = "output", help = "File to write formatted fasta.")]
    output_file: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    let result = match args.cmdline {
        Command::Cut(args) => fasta::edit::cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or(String::from("Could not cut")),
        Command::Generate(args) => fasta::make::generate(args.length, args.output_file).unwrap_or(String::from("Could not generate")),
        Command::Print(args) => fasta::view::cat(&args.file).unwrap_or(String::from("File not found")),
        Command::Format(args) => fasta::edit::format(args.file, args.uppercase, args.output_file).unwrap_or(String::from("Could not format")),
    };

    println!("{}", result);
}
