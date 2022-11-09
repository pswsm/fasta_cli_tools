use crate::{
    structs,
    dna2aa
};
use anyhow::Result;
use crate::fasta_ops::edit::format_str;
use crate::fasta::{Fasta, DNA_BASES, RNA_BASES};
use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread,
};

#[macro_export]
macro_rules! write2file {
    ($file:ident; $($fields:ident),+) => {{
        let mut output_file = File::create(&$file)?;
        $(
            output_file.write_all($fields.as_bytes())?;
         )+
    }}
}

/// Generates a RNA or DNA chain approximately N ```bases``` long. The actual length is different
/// because of how the parallellization is done.
pub fn generate(bases: usize, file: PathBuf, is_rna: bool) -> Result<String> {
    // let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let atcg: [&str; 4] = match is_rna {
        true => RNA_BASES,
        false => DNA_BASES,
    };

    let header: String = format!(">randomly generated sequence of {} bases\n", bases);

    let num_threads: usize = num_cpus::get();

    let sequence: String = match generate_bases(num_threads, bases, atcg) {
        Ok(seq) => seq.join("\n"),
        Err(e) => return Err(anyhow::anyhow!("Could not generate bases. Error: {:?}", e)),
    };
    let fmt_sequence: String = format_str(sequence).unwrap();
    write2file!(file; header, fmt_sequence);
    let result: String = format!("Generated file {} with {} bases", file.display(), bases);

    Ok(result)
}

fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String = String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}

fn generate_bases(num_threads: usize, num_bases: usize, bases: [&str; 4]) -> thread::Result<Vec<String>> {
    let bases_per_thread: usize = num_bases / num_threads;
    let base_list: Vec<String> = bases.iter().map(|b| b.to_string()).collect();
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut children: Vec<thread::JoinHandle<()>> = Vec::new();
    for _ in 0..num_threads {
        let bases_per_thread_copy: usize = bases_per_thread;
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
    for _ in 0..num_threads {
        sequences.push(rx.recv().unwrap_or_else(|_| select_rnd_str(&base_list) ));
        //println!("Message {} received", x);
    }

    Ok(sequences)
}

pub fn rev(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let rev_fasta: Fasta = {
        let rv: Fasta = fasta.reverse();
        let nfseq: String = rv.sequence;
        let fseq: String = format_str(nfseq).unwrap();
        Fasta::from(&[rv.header, fseq])
    };

    if let Some(..) = ofile {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write_all(fasta.header.as_bytes())?;
        output_file.write_all(fasta.sequence.as_bytes())?;
    }
    Ok(rev_fasta.to_string())
}

pub fn revcomp(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let revcomp_fasta: Fasta = {
        let rev: Fasta = fasta.reverse();
        let rvcmp: Fasta = rev.complement();
        let nfseq: String = rvcmp.sequence;
        let fseq: String = format_str(nfseq).unwrap();
        Fasta::from(&[rvcmp.header, fseq])
    };
    if let Some(..) = ofile {
        let ofile: PathBuf = ofile.unwrap();
        let header: String = revcomp_fasta.header.clone();
        let sequence: String = revcomp_fasta.sequence.clone();
        write2file!(ofile; header, sequence);
    }
    Ok(revcomp_fasta.to_string())
}

pub fn comp(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let comp_fasta: Fasta = {
        let cmp: Fasta = fasta.complement();
        let nfseq: String = cmp.sequence;
        let fseq: String = format_str(nfseq).unwrap();
        Fasta::from(&[cmp.header, fseq])
    };
    if let Some(..) = ofile {
        let ofile: PathBuf = ofile.unwrap();
        let header: String = comp_fasta.header.clone();
        let sequence: String = comp_fasta.sequence.clone();
        write2file!(ofile; header, sequence);
    }
    Ok(comp_fasta.to_string())
}

#[allow(dead_code)]
fn rev_f(file: PathBuf, ofile: Option<PathBuf>) -> Result<Fasta, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let rev_fasta: Fasta = fasta.reverse();
    if let Some(..) = ofile {
        let ofile: PathBuf = ofile.unwrap();
        let header: String = rev_fasta.header.clone();
        let sequence: String = rev_fasta.sequence.clone();
        write2file!(ofile; header, sequence);
    }
    Ok(rev_fasta)
}

#[allow(dead_code)]
fn revcomp_f(file: PathBuf, ofile: Option<PathBuf>) -> Result<Fasta, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let revcomp_fasta: Fasta = {
        let rev: Fasta = fasta.reverse();
        rev.complement()
    };
    if let Some(..) = ofile {
        let ofile: PathBuf = ofile.unwrap();
        let header: String = revcomp_fasta.header.clone();
        let sequence: String = revcomp_fasta.sequence.clone();
        write2file!(ofile; header, sequence);
    }
    Ok(revcomp_fasta)
}

#[allow(dead_code)]
fn comp_f(file: PathBuf, ofile: Option<PathBuf>) -> Result<Fasta, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let comp_fasta: Fasta = fasta.complement();
    if let Some(..) = ofile {
        let ofile: PathBuf = ofile.unwrap();
        let header: String = comp_fasta.header.clone();
        let sequence: String = comp_fasta.sequence.clone();
        write2file!(ofile; header, sequence);
    }
    Ok(comp_fasta)
}

pub fn to_aacids(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let aas: structs::ProteinChain = dna2aa::dna2aa(fasta.clone());
    if let Some(..) = ofile {
        let ofile: PathBuf = ofile.unwrap();
        let header: String = fasta.header.clone();
        let sequence: String = fasta.sequence;
        write2file!(ofile; header, sequence);
    }
    Ok(aas.to_string())
}

#[allow(dead_code)]
pub fn to_protein_chain(file: PathBuf) -> Result<structs::ProteinChain, anyhow::Error> {
    let fasta: Fasta = crate::view::cat_f(&file)?;
    let aas: structs::ProteinChain = dna2aa::dna2aa(fasta);
    Ok(aas)
}
