use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client};

mod car;
mod client;
mod error;
pub mod model;

pub use car::*;
pub use client::*;
pub use error::*;

#[derive(Debug, Clone)]
pub struct Repository {
    db: Surreal<Client>,
}

impl Repository {
    pub fn new(db: &Surreal<Client>) -> Self {
        Self { db: db.clone() }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PagingFilter {
    offset: u32,
    limit: u32,
}

impl PagingFilter {
    pub fn new(offset: u32, limit: u32) -> Self {
        Self { offset, limit }
    }
    pub fn from_path(offset: String, limit: String) -> Result<Self, RepositoryError> {
        Ok(Self {
            offset: offset
                .parse::<u32>()
                .map_err(|err| RepositoryError::ParsingError(err.to_string()))?,
            limit: limit
                .parse::<u32>()
                .map_err(|err| RepositoryError::ParsingError(err.to_string()))?,
        })
    }
}

impl Display for PagingFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LIMIT {} START {}", self.limit, self.offset)
    }
}

impl Default for PagingFilter {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 20,
        }
    }
}
