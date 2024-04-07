use crate::dna2aa;
use crate::fasta::{Fasta, DNA_BASES, RNA_BASES};
use crate::infrastructure::{write_chain_to_file, CommonWriteFormat};
use crate::structs::Protein;
use crate::utils::select_rnd_str;
use anyhow::Result;
use rayon::prelude::*;
use std::path::PathBuf;

pub enum FastaAllowedOperations {
    Reverse,
    Complement,
    Both,
}

/// Generates a RNA or DNA chain of N `bases` and saves it to `file`.
///
/// The String returned is not actually the sequence, but a info message about how the generation went.
pub fn generate(bases: usize, file: PathBuf, is_rna: bool) -> Result<String> {
    let atcg: [&str; 4] = match is_rna {
        true => RNA_BASES,
        false => DNA_BASES,
    };
    let header: String = format!("randomly generated sequence of {} bases", bases);
    let sequence: String = match generate_bases(bases, atcg) {
        Ok(seq) => seq.join(""),
        Err(e) => return Err(anyhow::anyhow!("Could not generate bases. Error: {:?}", e)),
    };
    let fsta = Fasta::from((header, sequence.clone()));
    write_chain_to_file(&file, CommonWriteFormat::from(fsta))?;
    let result: String = format!(
        "Generated file \"{}\" with {} bases",
        file.display(),
        sequence.len()
    );
    Ok(result)
}

/// Generates a random string chain given four different slices. Multithreaded if num_threads is bigger than one I guess
fn generate_bases(num_bases: usize, bases: [&str; 4]) -> Result<Vec<String>> {
    let base_list: Vec<String> = bases.iter().map(|b| b.to_string()).collect();
    let ray_seq: Vec<_> = (0..=num_bases)
        .into_par_iter()
        .map(|_| select_rnd_str(&base_list))
        .collect();
    Ok(ray_seq)
}

pub fn operate_on_chain(
    file: PathBuf,
    ofile: Option<PathBuf>,
    operation: FastaAllowedOperations,
) -> Result<String> {
    let original_fasta: Fasta = crate::view::cat_f(&file)?;
    let operated_fasta = match operation {
        FastaAllowedOperations::Reverse => original_fasta.reverse(),
        FastaAllowedOperations::Complement => original_fasta.complement(),
        FastaAllowedOperations::Both => original_fasta.reverse().complement(),
    };
    if ofile.is_some() {
        let ofile: PathBuf = ofile.unwrap();
        write_chain_to_file(&ofile, CommonWriteFormat::from(operated_fasta))?;
    }
    Ok("".to_string())
}

pub fn to_aacids(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let aas: Protein = dna2aa::fasta_to_protein(fasta);
    if ofile.is_some() {
        let ofile: PathBuf = ofile.unwrap();
        write_chain_to_file(&ofile, CommonWriteFormat::from(aas))?;
    }
    Ok("".to_string())
}
