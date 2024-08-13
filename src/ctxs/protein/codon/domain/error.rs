use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodonError {
    #[error("Invalid base")]
    InvalidBases,
}
