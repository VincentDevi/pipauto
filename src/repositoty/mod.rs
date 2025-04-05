use surrealdb::{Surreal, engine::remote::ws::Client};

mod client;
mod car;
mod error;
pub mod model;

pub use client::*;
pub use car::*;
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
