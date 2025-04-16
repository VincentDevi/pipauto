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
    pub fn offset(&self) -> u32 {
        self.offset
    }
    pub fn limit(&self) -> u32 {
        self.limit
    }
    pub fn increment_paging(&self, value: u32) -> Self {
        Self {
            offset: self.offset + value,
            limit: self.limit,
        }
    }
    pub fn decrement_paging(&self, value: u32) -> Self {
        let offset = if value > self.offset {
            0
        } else {
            self.offset - value
        };
        Self {
            offset,
            limit: self.limit,
        }
    }
    pub fn reset(&self) -> Self {
        Self {
            offset: 0,
            limit: self.limit(),
        }
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
            limit: 2,
        }
    }
}
