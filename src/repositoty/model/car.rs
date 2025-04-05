use super::client::ModelClient;
use strum_macros::Display;
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone)]
pub struct ModelCar {
    client_id: RecordId,
    brand: String,
    cc: f64,
    oil_type: String,
    oil_quantity: f64,
    year: u32,
    fuel: ModelFuel,
    model: String,
    created_at: Datetime,
    updated_at: Datetime,
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ModelFuel {
    Gasoline,
    Diesel,
    Other,
}
