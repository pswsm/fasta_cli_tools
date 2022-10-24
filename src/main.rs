mod fasta_ops;
mod fasta;
mod structs;
mod dna2aa;
mod args;
use fasta_ops::{edit, view, make};
use args::{
    Command,
    Args,
    FastaOperation
};
use structopt::StructOpt;

// ----------------

/// Runs the program
fn main() {
    let args = Args::from_args();

    let result = match args.cmdline {
        Command::Cut(args)      => edit::cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or_else(|_| String::from("Could not cut")),
        Command::Generate(args) => make::generate(args.length, args.output_file, args.is_rna).unwrap_or_else(|_| String::from("Could not generate")),
        Command::Print(args)    => view::cat(&args.file).unwrap_or_else(|_| String::from("Could not print file")),
        Command::Format(args)   => edit::format(args.file, args.uppercase, args.output_file).unwrap_or_else(|_| String::from("Could not format")),
        Command::Analyze(args)  => view::analize(&args.file).unwrap_or_else(|_| String::from("Could not analyze")),
        Command::Get(args) => match args {
            FastaOperation::Reverse(fst) => make::rev(fst.file, fst.ofile).unwrap_or_else(|_| String::from("Could not get reverse strand")),
            FastaOperation::Complementary(fst) => make::comp(fst.file, fst.ofile).unwrap_or_else(|_| String::from("Could not get complementary strand")),
            FastaOperation::Revcomp(fst) => make::revcomp(fst.file, fst.ofile).unwrap_or_else(|_| String::from("Could not get reverse-complementary strand")),
            FastaOperation::Amioacids(fst) => make::to_aacids(fst.file, fst.ofile).unwrap_or_else(|_| String::from("Could not convert to aminoacids"))
        },
    };

    println!("{}", result);
}
