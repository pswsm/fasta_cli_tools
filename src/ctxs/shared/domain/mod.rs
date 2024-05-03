pub enum ChainTypes {
    DNA,
    RNA,
    PROTEIN,
}

pub type ChainObject = Vec<String>;

pub trait Chain {
    fn chain_type(&self) -> ChainTypes;
}
