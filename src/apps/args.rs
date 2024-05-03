//! All argument definitions

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Base `command` for arguments.
#[derive(Parser)]
#[command(
    name = "fasta_cli_toolkit",
    about = "A CLI toolkit to manipulate fasta files",
    rename_all = "kebab-case"
)]
pub struct Arguments {
    #[command(subcommand)]
    pub cmdline: Command,
}

// ----------------

/// Enumeration with all possible command options.
#[derive(Parser)]
pub enum Command {
    Print(CatOptions),
    Cut(CutOptions),
    Generate(GenerateOptions),
    Format(FormatOptions),
    Analyze(AnalysisOptions),
    #[command(subcommand)]
    Get(FastaOperation),
}

// ----------------

/// Available arguments for `CatOptions`.
#[derive(Parser)]
#[command(
    name = "print file options",
    about = "Reads file, prints its content",
    rename_all = "kebab-case"
)]
pub struct CatOptions {
    #[arg(help = "The file to read")]
    pub file: PathBuf,
}

// ----------------

/// Available arguments for `CutOptions`.
#[derive(Parser)]
#[command(
    name = "cutting options",
    about = "Cuts nucleotides from..to range",
    rename_all = "kebab-case"
)]
pub struct CutOptions {
    #[arg(help = "Position to start cutting")]
    pub from: usize,

    #[arg(help = "Position to start cutting")]
    pub to: usize,

    #[arg(help = "File to read")]
    pub input_file_name: PathBuf,

    #[arg(help = "File to write")]
    pub output_file_name: PathBuf,
}

// ----------------

/// Available options for `GenerateOptions`.
#[derive(Parser)]
#[command(
    name = "generation options",
    about = "Generates a fasta file of n bases",
    rename_all = "kebab-case"
)]
pub struct GenerateOptions {
    #[arg(help = "Number of bases to generate. Each line has 60 bases")]
    pub length: usize,

    #[arg(help = "File to write to")]
    pub output_file: PathBuf,

    #[arg(short = 'r', long = "rna", help = "Generate RNA instead of DNA")]
    pub is_rna: bool,
}

// ----------------

/// Available options for `FormatOptions`.
#[derive(Parser)]
#[command(
    name = "format otions",
    about = "Formats a fasta file",
    rename_all = "kebab-case"
)]
pub struct FormatOptions {
    #[arg(help = "File to format")]
    pub file: PathBuf,

    #[arg(help = "File to write formatted fasta. Optional")]
    pub output_file: PathBuf,

    #[arg(short, long = "upper", help = "Format to uppercase")]
    pub uppercase: bool,
}

// ----------------

/// Available options for `AnalysisOptions`.
#[derive(Parser)]
#[command(
    name = "analysis options",
    about = "Analyzes a sequence",
    rename_all = "kebab-case"
)]
pub struct AnalysisOptions {
    #[arg(help = "File to analize")]
    pub file: PathBuf,
}

// ----------------

#[derive(Subcommand)]
#[command(
    name = "sequence related operations",
    about = "Get reverse, completary, reverse-complementary or transform to amioacids",
    rename_all = "kebab-case"
)]
pub enum FastaOperation {
    Reverse(StrandOptions),
    Complementary(StrandOptions),
    ReverseComplementary(StrandOptions),
    Amioacids(AAOptions),
}

// ----------------

/// Available options for `StrandOptions`.
#[derive(Parser)]
#[command(
    name = "get strand from another strand",
    about = "Get a strand, be it the reverse, the completary or the reverse-complementary from a given fasta file",
    rename_all = "kebab-case"
)]
pub struct StrandOptions {
    #[arg(help = "File to read from")]
    pub file: PathBuf,
    #[arg(help = "File to write to")]
    pub ofile: Option<PathBuf>,
}

/// Available options for `AAOptions`.
#[derive(Parser)]
#[command(
    name = "transform rna to aa",
    about = "Transform a RNA sequence to an Amioacid one.",
    rename_all = "kebab-case"
)]
pub struct AAOptions {
    #[arg(help = "File to read from")]
    pub file: PathBuf,
    #[arg(help = "File to write to")]
    pub ofile: Option<PathBuf>,
    #[arg(short, long, help = "Protein in lowercase (default: false)")]
    pub lowercase: bool,
}
