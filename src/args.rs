//! All argument definitions

use std::path::PathBuf;
use structopt::StructOpt;

/// Base `structopt` for arguments.
#[derive(StructOpt)]
#[structopt(name = "fasta_cli_toolkit",
            about = "A CLI toolkit to manipulate fasta files",
            rename_all = "kebab-case")]
pub struct Args {
    #[structopt(subcommand)]
    pub cmdline: Command
}

// ----------------

/// Enumeration with all possible command options.
#[derive(StructOpt)]
pub enum Command {
    Print(CatOptions),
    Cut(CutOptions),
    Generate(GenerateOptions),
    Format(FormatOptions),
    Analyze(AnalysisOptions),
    Get(FastaOperation)
}

// ----------------

/// Available arguments for `CatOptions`.
#[derive(StructOpt)]
#[structopt(name = "print file options",
            about = "Reads file, prints its content",
            rename_all = "kebab-case")]
pub struct CatOptions {
    #[structopt(help = "The file to read")]
    pub file: PathBuf,
}

// ----------------

/// Available arguments for `CutOptions`.
#[derive(StructOpt)]
#[structopt(name = "cutting options",
            about = "Cuts nucleotides from..to range",
            rename_all = "kebab-case")]
pub struct CutOptions {
    #[structopt(help = "Position to start cutting")]
    pub from: usize,
    
    #[structopt(help = "Position to start cutting")]
    pub to: usize,

    #[structopt(help = "File to read")] 
    pub input_file_name: PathBuf,

    #[structopt(help = "File to write")]
    pub output_file_name: PathBuf,    
}

// ----------------

/// Available options for `GenerateOptions`.
#[derive(StructOpt)]
#[structopt(name = "generation options",
            about = "Generates a fasta file of n bases",
            rename_all = "kebab-case")]
pub struct GenerateOptions {
    #[structopt(help = "Number of bases to generate. Each line has 60 bases")] 
    pub length: usize,

    #[structopt(help = "File to write to")]
    pub output_file: PathBuf,

    #[structopt(short = "r", long = "rna", help = "Generate RNA instead of DNA")]
    pub is_rna: bool,
}

// ----------------

/// Available options for `FormatOptions`.
#[derive(StructOpt)]
#[structopt(name = "format otions",
            about = "Formats a fasta file",
            rename_all = "kebab-case")]
pub struct FormatOptions {
    #[structopt(help = "File to format")]
    pub file: PathBuf,

    #[structopt(help = "File to write formatted fasta. Optional")]
    pub output_file: PathBuf,
    
    #[structopt(short, long = "upper", help = "Format to uppercase")]
    pub uppercase: bool,

}

// ----------------

/// Available options for `AnalysisOptions`.
#[derive(StructOpt)]
#[structopt(name = "analysis options",
            about = "Analyzes a sequence",
            rename_all = "kebab-case")]
pub struct AnalysisOptions {
    #[structopt(help = "File to analize")]
    pub file: PathBuf,
}

// ----------------

#[derive(StructOpt)]
pub enum FastaOperation {
    Reverse(StrandOptions),
    Complementary(StrandOptions),
    Revcomp(StrandOptions),
    Amioacids(AAOptions)
}

// ----------------

/// Available options for `StrandOptions`.
#[derive(StructOpt)]
#[structopt(name = "get strand from another strand",
            about = "Get a strand, be it the reverse, the completary or the reverse-complementary from a given fasta file",
            rename_all = "kebab-case")]
pub struct StrandOptions {
    #[structopt(help = "File to read from")]
    pub file: PathBuf,
    #[structopt(help = "File to write to")]
    pub ofile: Option<PathBuf>
}

/// Available options for `AAOptions`.
#[derive(StructOpt)]
#[structopt(name = "transform rna to aa",
            about = "Transform a RNA sequence to an Amioacid one.",
            rename_all = "kebab-case")]
pub struct AAOptions {
    #[structopt(help = "File to read from")]
    pub file: PathBuf,
    #[structopt(help = "File to write to")]
    pub ofile: Option<PathBuf>,
    #[structopt(short, long, help = "Protein in lowercase (default: false)")]
    pub lowercase: bool
}
