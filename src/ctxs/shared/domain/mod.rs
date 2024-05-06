pub enum SequenceType {
    Dna,
    Rna,
    Protein,
}

pub type SequenceObject = Vec<String>;

pub trait Sequence {
    fn sequence_type(&self) -> SequenceType;
}
