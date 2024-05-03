pub enum SequenceTypes {
    DNA,
    RNA,
    PROTEIN,
}

pub type SequenceObject = Vec<String>;

pub trait Sequence {
    fn sequence_type(&self) -> SequenceTypes;
}
