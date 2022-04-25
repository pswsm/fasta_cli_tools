use rand::seq::SliceRandom;
use std::{
    fs::File,
    path::PathBuf,
    io::prelude::Write,
    thread,
    sync::mpsc,
    sync::mpsc::{
        Sender,
        Receiver
    },
};
use crate::fasta_ops::edit::format_str;

pub fn generate(bases: usize, file: PathBuf) -> std::io::Result<String> {
    let atcg: Vec<String> = vec![String::from("a"), String::from("t"), String::from("c"), String::from("g")];
    let header: String = format!(">randomly generated sequence of {} bases\n", bases);

    let num_threads: usize = bases / 1000;

    let sequence: String = match spawn_threads(num_threads, bases, atcg){
        Ok(seq) => seq.join("\n"),
        Err(e) => panic!("Could not generate bases. Error: {:?}", e)
    };

    let fmt_sequence: String = match format_str(sequence) {
        Ok(seq) => seq,
        Err(e) => panic!("Could not format. Error {}", e)
    };
    let mut output_file = File::create(&file)?;
    output_file.write(header.as_bytes())?;
    output_file.write(fmt_sequence.as_bytes())?;

    let result: String = format!("Generated file {} with {} bases", file.display(), bases);

    Ok(result)
}

//#[derive(Clone, Copy)]
fn select_rnd_str(string_list: &Vec<String>) -> String {
    let selected_string: String = String::from(string_list.choose(&mut rand::thread_rng()).unwrap());
    selected_string
}

fn spawn_threads(num_threads: usize, num_bases: usize, bases: Vec<String>) -> thread::Result<Vec<String>> {
    let bases_per_thread: usize = num_bases / num_threads;
    let base_list: Vec<String> = bases;
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut children: Vec<thread::JoinHandle<()>> = Vec::new();
    for _ in 0..num_threads {
        let bases_per_thread_copy: usize = bases_per_thread.clone();
        let thread_tx: Sender<String> = tx.clone();
        let base_list_copy: Vec<String> = base_list.clone();
        let child = thread::spawn(move || {
            let sequence: String = (0..bases_per_thread_copy).map(|_| select_rnd_str(&base_list_copy)).collect(); 
            thread_tx.send(sequence).unwrap();
        });
        children.push(child);
    };

    let mut sequences: Vec<String> = Vec::new();
    for _ in 0..num_threads {
        sequences.push(rx.recv().unwrap());
    };

    for child in children {
        child.join().unwrap();
    };

    Ok(sequences)
}
