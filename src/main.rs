mod fasta_ops;
mod args;
use fasta_ops::{edit, view, make};
use args::{
    Command,
    Args,
    FastaOperation
};
use structopt::StructOpt;

// ----------------

fn main() {
    let args = Args::from_args();

    let result = match args.cmdline {
        Command::Cut(args)      => edit::cutting(args.input_file_name, args.output_file_name, args.from, args.to).unwrap_or(String::from("Could not cut")),
        Command::Generate(args) => make::generate(args.length, args.output_file, args.is_rna).unwrap_or(String::from("Could not generate")),
        Command::Print(args)    => view::cat(&args.file).unwrap_or(String::from("Could not print file")),
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
