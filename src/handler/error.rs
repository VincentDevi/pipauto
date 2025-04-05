use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Repository Error: {0}")]
    Repository(String),
    #[error("Parsing Error: {0}")]
    Parsing(String),
}
