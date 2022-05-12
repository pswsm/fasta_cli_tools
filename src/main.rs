mod fasta_ops;
use fasta_ops::{edit, view, make};

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
    Format(FormatOptions),
    Analyze(AnalysisOptions),
    Get(FastaOperation)
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
            about = "Generates a fasta file of n bases",
            rename_all = "kebab-case")]
struct GenerateOptions {
    #[structopt(help = "Number of bases to generate. Each line has 60 bases")] 
    length: usize,

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

    #[structopt(help = "File to write formatted fasta. Optional")]
    output_file: PathBuf,
    
    #[structopt(short, long = "upper", help = "Format to uppercase")]
    uppercase: bool,

}

#[derive(StructOpt)]
#[structopt(name = "analysis options",
            about = "Analyzes a sequence",
            rename_all = "kebab-case")]
struct AnalysisOptions {
    #[structopt(help = "File to analize")]
    file: PathBuf,
}

#[derive(StructOpt)]
enum FastaOperation {
    Reverse(ReverseOptions),
    Complementary(ComplementOptions),
    Revcomp(RevCompOptions)
}

#[derive(StructOpt)]
#[structopt(name = "get reverse strand from given fasta",
            about = "Get the reverse strand from direct strand.",
            rename_all = "kebab-case")]
struct ReverseOptions {
    #[structopt(help = "File to read from")]
    file: PathBuf,
    #[structopt(help = "File to write to")]
    ofile: Option<PathBuf>
}

#[derive(StructOpt)]
#[structopt(name = "get reverse-complementary strand from given fasta",
            about = "Get the reverse-complementary strand from direct strand.",
            rename_all = "kebab-case")]
struct RevCompOptions {
    #[structopt(help = "File to read from")]
    file: PathBuf,
    #[structopt(help = "File to write to")]
    ofile: Option<PathBuf>
}

#[derive(StructOpt)]
#[structopt(name = "get complementary strand from given fasta",
            about = "Get the complementary strand from direct strand.",
            rename_all = "kebab-case")]
struct ComplementOptions {
    #[structopt(help = "File to read from")]
    file: PathBuf,
    #[structopt(help = "File to write to")]
    ofile: Option<PathBuf>
}

fn main() {
    let args = Args::from_args();

    let result = match args.cmdline {
        Command::Cut(args)      => edit::cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or(String::from("Could not cut")),
        Command::Generate(args) => make::generate(args.length, args.output_file).unwrap_or(String::from("Could not generate")),
        Command::Print(args)    => view::cat_as_string(&args.file).unwrap_or(String::from("Could not print file")),
        Command::Format(args)   => edit::format(args.file, args.uppercase, args.output_file).unwrap_or(String::from("Could not format")),
        Command::Analyze(args)  => view::analize(&args.file).unwrap_or(String::from("Could not analyze")),
        Command::Get(args) => match args {
            FastaOperation::Reverse(fst) => make::rev(fst.file, fst.ofile).unwrap_or(String::from("Could not get reverse strand")),
            FastaOperation::Complementary(fst) => make::comp(fst.file, fst.ofile).unwrap_or(String::from("Could not get complementary strand")),
            FastaOperation::Revcomp(fst) => make::revcomp(fst.file, fst.ofile).unwrap_or(String::from("Could not get reverse-complementary strand"))
        },
    };

    println!("{}", result);
}
