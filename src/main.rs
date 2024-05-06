#![warn(clippy::pedantic, clippy::complexity, clippy::perf)]
mod apps;
mod ctxs;
use apps::args::{Arguments, Command, FastaOperation};
use clap::Parser;
use ctxs::fasta::application::{chain_operations, cut, format, generate, to_aminoacids, view};

// ----------------

/// Runs the program
fn main() {
    let args = Arguments::parse();

    let result = match args.cmdline {
        Command::Cut(args) => cut::cut(
            args.input_file_name,
            args.output_file_name,
            args.from,
            args.to,
        )
        .unwrap_or_else(|_| String::from("Could not cut")),
        Command::Generate(args) => generate::generate(args.length, args.output_file, args.is_rna)
            .unwrap_or_else(|_| String::from("Could not generate")),
        Command::Print(args) => {
            view::cat(&args.file).unwrap_or_else(|_| String::from("Could not print file"))
        }
        Command::Format(args) => format::format(args.file, args.uppercase, args.output_file)
            .unwrap_or_else(|_| String::from("Could not format")),
        Command::Analyze(args) => {
            view::analize(&args.file).unwrap_or_else(|_| String::from("Could not analyze"))
        }
        Command::Get(args) => match args {
            FastaOperation::Reverse(fst) => chain_operations::reverse(fst)
                .unwrap_or_else(|_| String::from("Could not get reverse strand")),
            FastaOperation::Complementary(fst) => chain_operations::complementary(fst)
                .unwrap_or_else(|_| String::from("Could not get complementary strand")),
            FastaOperation::ReverseComplementary(fst) => {
                chain_operations::reverse_complementary(fst)
                    .unwrap_or_else(|_| String::from("Could not get reverse-complementary strand"))
            }
            FastaOperation::Amioacids(fst) => to_aminoacids::to_aminoacids(fst.file, fst.ofile)
                .unwrap_or_else(|_| String::from("Could not convert to aminoacids")),
        },
    };

    println!("{}", result);
}
