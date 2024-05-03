use std::path::PathBuf;

use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::ctxs::{
    fasta::domain::fasta::{Fasta, DNA_BASES, RNA_BASES},
    shared::utils::select_rnd_str,
};

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
    fsta.save(&file)?;
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
