use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use std::sync::PoisonError;
use templating::error::Error as TemplatingError;
use thiserror::Error;

use crate::repositoty::RepositoryError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,
    #[error("internal error: {0}")]
    Internal(String),
    #[error("repository error: {0}")]
    Repository(String),
    #[error("templating error: {0}")]
    Templating(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Self::Internal("Mutex lock poisoned".into())
    }
}

impl From<RepositoryError> for Error {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error.to_string())
    }
}

impl From<TemplatingError> for Error {
    fn from(value: TemplatingError) -> Self {
        Self::Templating(value.to_string())
    }
}
