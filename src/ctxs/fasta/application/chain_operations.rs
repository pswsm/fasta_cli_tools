use anyhow::Result;

use crate::{apps::args::StrandOptions, ctxs::fasta::domain::fasta::Fasta};

use super::view;

pub fn complementary(options: StrandOptions) -> Result<String> {
    let fasta: Fasta = view::cat_f(&options.file).unwrap().complement();
    if let Some(file) = options.ofile {
        fasta.save(&file)?;
    }
    Ok("".to_string())
}

pub fn reverse(options: StrandOptions) -> Result<String> {
    let fasta: Fasta = view::cat_f(&options.file).unwrap().reverse();
    if let Some(file) = options.ofile {
        fasta.save(&file)?;
    }
    Ok("".to_string())
}

pub fn reverse_complementary(options: StrandOptions) -> Result<String> {
    let fasta: Fasta = view::cat_f(&options.file).unwrap().reverse().complement();
    if let Some(file) = options.ofile {
        fasta.save(&file)?;
    }
    Ok("".to_string())
}
