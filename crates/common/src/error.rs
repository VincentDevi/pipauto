use std::sync::Arc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parsing error: {0}")]
    Parsing(Arc<str>),
}
