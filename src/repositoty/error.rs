use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("Database Error")]
    DatabaseError,
}
