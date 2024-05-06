pub enum SequenceType {
    Dna,
    Rna,
    Protein,
}

pub type SequenceObject<T> = Vec<T>;

pub trait Sequence<T> {
    fn sequence_type(&self) -> SequenceType;
    fn get_chain(&self) -> SequenceObject<T>;
}
