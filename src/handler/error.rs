use thiserror::Error;

use crate::repositoty::RepositoryError;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Repository Error: {0}")]
    Repository(String),
    #[error("Parsing Error: {0}")]
    Parsing(String),
}

impl From<RepositoryError> for HandlerError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value.to_string())
    }
}