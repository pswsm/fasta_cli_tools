pub struct Fasta {
    pub header: String,
    pub sequence: String
}

#[allow(dead_code)]
impl Fasta {
    pub const fn new() -> Fasta {
        Fasta { header: String::new(), sequence: String::new() }
    }

    pub const fn from(headr: String, seqnc: String) -> Fasta {
        Fasta { header: headr, sequence: seqnc }
    }

    pub fn to_string(&self) -> String {
        String::from(format!("{}\n{}", self.header, self.sequence))
    }
}
