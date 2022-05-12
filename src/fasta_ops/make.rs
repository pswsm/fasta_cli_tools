use rand::seq::SliceRandom;
use std::{
    fs::File,
    path::PathBuf,
    io,
    io::prelude::Write,
    thread,
    sync::mpsc,
    sync::mpsc::{
        Sender,
        Receiver
    },
};
use crate::fasta_ops::{
    edit::format_str,
    fasta::Fasta,
};

pub fn generate(bases: usize, file: PathBuf) -> std::io::Result<String> {
    let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let header: String = format!(">randomly generated sequence of {} bases\n", bases);

    let num_threads: usize = num_cpus::get();

    //println!("Spawning threads");
    let sequence: String = match generate_bases(num_threads, bases, atcg){
        Ok(seq) => seq.join("\n"),
        Err(e) => panic!("Could not generate bases. Error: {:?}", e)
    };
    //println!("Spawning finished.");

    //println!("Starting sequence formting.");
    let fmt_sequence: String = match format_str(sequence) {
        Ok(seq) => seq,
        Err(e) => panic!("Could not format. Error {}", e)
    };
    //println!("Finished formatting");

    //println!("Writing file.");
    let mut output_file = File::create(&file)?;
    output_file.write(header.as_bytes())?;
    output_file.write(fmt_sequence.as_bytes())?;
    //println!("Finish writing file");

    let result: String = format!("Generated file {} with {} bases", file.display(), bases);

    Ok(result)
}

//#[derive(Clone, Copy)]
fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String = String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}

fn generate_bases(num_threads: usize, num_bases: usize, bases: Vec<String>) -> thread::Result<Vec<String>> {
    let bases_per_thread: usize = num_bases / num_threads;
    let base_list: Vec<String> = bases;
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut children: Vec<thread::JoinHandle<()>> = Vec::new();
    for _ in 0..num_threads {
        let bases_per_thread_copy: usize = bases_per_thread.clone();
        let thread_tx: Sender<String> = tx.clone();
        let base_list_copy: Vec<String> = base_list.clone();
        let child = thread::spawn(move || {
            //println!("Starting hread number {}", i);
            let sequence: String = (0..bases_per_thread_copy).map(|_| select_rnd_str(&base_list_copy)).collect(); 
            thread_tx.send(sequence).unwrap();
            //println!("Thread {} just sent its message", i);
        });
        children.push(child);
    };

    let mut sequences: Vec<String> = Vec::new();
    for _ in 0..num_threads {
        sequences.push(rx.recv().unwrap());
        //println!("Message {} received", x);
    };

    Ok(sequences)
}

pub fn rev(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, io::Error> {
    let fasta: Fasta = crate::view::cat(&file)?;
    let rev_fasta: Fasta = {
        let rv: Fasta = fasta.reverse();
        let nfseq: String = rv.sequence;
        let fseq: String = format_str(nfseq).unwrap();
        Fasta::from(rv.header, fseq)
    };

    if ofile.is_some() {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write(fasta.header.as_bytes())?;
        output_file.write(fasta.sequence.as_bytes())?;
    }
    Ok(rev_fasta.to_string())
}

pub fn revcomp(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, io::Error> {
    let fasta: Fasta = crate::view::cat(&file)?;
    let revcomp_fasta: Fasta = {
        let rev: Fasta = fasta.reverse();
        let rvcmp: Fasta = rev.complement();
        let nfseq: String = rvcmp.sequence;
        let fseq: String = format_str(nfseq).unwrap();
        Fasta::from(rvcmp.header, fseq)
    };
    if ofile.is_some() {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write(fasta.header.as_bytes())?;
        output_file.write(fasta.sequence.as_bytes())?;
    }
    Ok(revcomp_fasta.to_string())
}

pub fn comp(file: PathBuf, ofile: Option<PathBuf>) -> Result<String, io::Error> {
    let fasta: Fasta = crate::view::cat(&file)?;
    let comp_fasta: Fasta = {
        let cmp: Fasta = fasta.complement();
        let nfseq: String = cmp.sequence;
        let fseq: String = format_str(nfseq).unwrap();
        Fasta::from(cmp.header, fseq)
    };
    if ofile.is_some() {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write(fasta.header.as_bytes())?;
        output_file.write(fasta.sequence.as_bytes())?;
    }
    Ok(comp_fasta.to_string())
}

#[allow(dead_code)]
fn rev_f(file: PathBuf, ofile: Option<PathBuf>) -> Result<Fasta, io::Error> {
    let fasta: Fasta = crate::view::cat(&file)?;
    let rev_fasta: Fasta = fasta.reverse();
    if ofile.is_some() {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write(fasta.header.as_bytes())?;
        output_file.write(fasta.sequence.as_bytes())?;
    }
    Ok(rev_fasta)
}

#[allow(dead_code)]
fn revcomp_f(file: PathBuf, ofile: Option<PathBuf>) -> Result<Fasta, io::Error> {
    let fasta: Fasta = crate::view::cat(&file)?;
    let revcomp_fasta: Fasta = {
        let rev: Fasta = fasta.reverse();
        rev.complement()
    };
    if ofile.is_some() {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write(fasta.header.as_bytes())?;
        output_file.write(fasta.sequence.as_bytes())?;
    }
    Ok(revcomp_fasta)
}

#[allow(dead_code)]
fn comp_f(file: PathBuf, ofile: Option<PathBuf>) -> Result<Fasta, io::Error> {
    let fasta: Fasta = crate::view::cat(&file)?;
    let comp_fasta: Fasta = fasta.complement();
    if ofile.is_some() {
        let mut output_file = File::create(&ofile.unwrap())?;
        output_file.write(fasta.header.as_bytes())?;
        output_file.write(fasta.sequence.as_bytes())?;
    }
    Ok(comp_fasta)
}
