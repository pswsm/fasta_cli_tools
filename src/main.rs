mod apps;
mod ctxs;
use apps::args::{Arguments, Command, FastaOperation};
use clap::Parser;
use ctxs::fasta::application::{edit, make, view};
mod shared;

// ----------------

/// Runs the program
fn main() {
    let args = Arguments::parse();

    let result = match args.cmdline {
        Command::Cut(args) => edit::cut(
            args.input_file_name,
            args.output_file_name,
            args.from,
            args.to,
        )
        .unwrap_or_else(|_| String::from("Could not cut")),
        Command::Generate(args) => make::generate(args.length, args.output_file, args.is_rna)
            .unwrap_or_else(|_| String::from("Could not generate")),
        Command::Print(args) => {
            view::cat(&args.file).unwrap_or_else(|_| String::from("Could not print file"))
        }
        Command::Format(args) => edit::format(args.file, args.uppercase, args.output_file)
            .unwrap_or_else(|_| String::from("Could not format")),
        Command::Analyze(args) => {
            view::analize(&args.file).unwrap_or_else(|_| String::from("Could not analyze"))
        }
        Command::Get(args) => match args {
            FastaOperation::Reverse(fst) => {
                make::operate_on_chain(fst.file, fst.ofile, make::FastaAllowedOperations::Reverse)
                    .unwrap_or_else(|_| String::from("Could not get reverse strand"))
            }
            FastaOperation::Complementary(fst) => make::operate_on_chain(
                fst.file,
                fst.ofile,
                make::FastaAllowedOperations::Complement,
            )
            .unwrap_or_else(|_| String::from("Could not get complementary strand")),
            FastaOperation::Revcomp(fst) => {
                make::operate_on_chain(fst.file, fst.ofile, make::FastaAllowedOperations::Both)
                    .unwrap_or_else(|_| String::from("Could not get reverse-complementary strand"))
            }
            FastaOperation::Amioacids(fst) => make::to_aacids(fst.file, fst.ofile)
                .unwrap_or_else(|_| String::from("Could not convert to aminoacids")),
        },
    };

    println!("{}", result);
}
