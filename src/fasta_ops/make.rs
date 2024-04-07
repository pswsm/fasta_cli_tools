use crate::dna2aa;
use crate::fasta::{Fasta, DNA_BASES, RNA_BASES};
use crate::fasta_ops::edit::format_str;
use crate::infrastructure::{write_chain_to_file, CommonWriteFormat};
use crate::structs::Protein;
use anyhow::Result;
use rand::seq::SliceRandom;
use std::{
    path::PathBuf,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread,
};

/// Generates a RNA or DNA chain of N `bases` and saves it to `file`.
///
/// The String returned is not actually the sequence, but a info message about how the generation went.
pub fn generate(bases: usize, file: PathBuf, is_rna: bool) -> Result<String> {
    // let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let atcg: [&str; 4] = match is_rna {
        true => RNA_BASES,
        false => DNA_BASES,
    };
    let filename = file.display();
    let header: String = format!("randomly generated sequence of {} bases", bases);
    let num_threads: usize = num_cpus::get();
    let sequence: String = match generate_bases(num_threads, bases, atcg) {
        Ok(seq) => seq.join("\n"),
        Err(e) => return Err(anyhow::anyhow!("Could not generate bases. Error: {:?}", e)),
    };
    let (fixed_length_sequence, _): (&str, _) = sequence.split_at(bases);
    let fmt_sequence: String = format_str(fixed_length_sequence.to_string()).unwrap();
    let fsta = Fasta::from((header, fmt_sequence));
    write_chain_to_file(&file, CommonWriteFormat::from(fsta))?;
    let result: String = format!(
        "Generated file \"{}\" with {} bases",
        filename,
        fixed_length_sequence.to_string().len()
    );

    Ok(result)
}

/// Select a random `String` from a given `Vector`.
fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String =
        String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}

/// Generates a random string chain given four different slices. Multithreaded if num_threads is bigger than one I guess
fn generate_bases(
    threads: usize,
    num_bases: usize,
    bases: [&str; 4],
) -> thread::Result<Vec<String>> {
    let bases_per_thread: usize = num_bases / threads;
    let bases_diff: usize = num_bases - bases_per_thread;
    let base_list: Vec<String> = bases.iter().map(|b| b.to_string()).collect();
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut children: Vec<thread::JoinHandle<()>> = Vec::new();
    for thread in 0..threads {
        let mut bases_per_thread_copy: usize = bases_per_thread;
        if thread >= threads {
            bases_per_thread_copy += bases_diff;
        };
        let thread_tx: Sender<String> = tx.clone();
        let base_list_copy: Vec<String> = base_list.clone();
        let child = thread::spawn(move || {
            //println!("Starting hread number {}", i);
            let sequence: String = (0..bases_per_thread_copy)
                .map(|_| select_rnd_str(&base_list_copy))
                .collect();
            let fmt_sequence: String = format_str(sequence).unwrap();
            thread_tx.send(fmt_sequence).unwrap();
            //println!("Thread {} just sent its message", i);
        });
        children.push(child);
    }

    let mut sequences: Vec<String> = Vec::new();
    for _ in 0..threads {
        sequences.push(rx.recv().unwrap_or_else(|_| select_rnd_str(&base_list)));
        //println!("Message {} received", x);
    }
    Ok(sequences)
}

pub fn rev(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let original_fasta: Fasta = crate::view::cat_f(&file)?;
    let rev_fasta: Fasta = original_fasta.reverse();
    if ofile.is_some() {
        let ofile: PathBuf = ofile.unwrap();
        write_chain_to_file(&ofile, CommonWriteFormat::from(rev_fasta))?;
    }
    Ok("".to_string())
}

pub fn revcomp(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let revcomp_fasta: Fasta = {
        let rev: Fasta = fasta.reverse();
        rev.complement()
    };
    if ofile.is_some() {
        let ofile: PathBuf = ofile.unwrap();
        write_chain_to_file(&ofile, CommonWriteFormat::from(revcomp_fasta))?;
    }
    Ok("".to_string())
}

pub fn comp(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let comp_fasta: Fasta = fasta.complement();
    if ofile.is_some() {
        let ofile: PathBuf = ofile.unwrap();
        write_chain_to_file(&ofile, CommonWriteFormat::from(comp_fasta))?;
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
