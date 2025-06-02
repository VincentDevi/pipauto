use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::repositoty::RepositoryError;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Repository Error: {0}")]
    Repository(String),
    #[error("Parsing Error: {0}")]
    Parsing(String),
    #[error("Templating Error: {0}")]
    Templating(String),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let status = match self {
            HandlerError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::Templating(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::Parsing(_) => StatusCode::BAD_REQUEST,
        };

        (status, Json(self.to_string())).into_response()
    }
}

impl From<RepositoryError> for HandlerError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value.to_string())
    }
}
