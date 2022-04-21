pub struct Fasta {
    pub header: String,
    pub sequence: String
}

impl Fasta {
    #[allow(dead_code)]
    pub const fn new() -> Fasta {
        Fasta { header: String::new(), sequence: String::new() }
    }

    pub const fn from(headr: String, seqnc: String) -> Fasta {
        Fasta { header: headr, sequence: seqnc }
    }
}
