use thiserror::Error;

#[derive(Error, Debug)]
pub enum AminoacidErrors {
    #[error("tried to create non-existing aminoacid")]
    NonExistingAminoacidError,
}
