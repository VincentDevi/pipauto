use strum_macros::Display;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCar {
    client_id: RecordId,
    brand: String,
    cc: String,
    oil_type: String,
    oil_quantity: String,
    year: u32,
    fuel: ModelFuel,
    model: String,
    created_at: Datetime,
    updated_at: Datetime,
}

#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize)]
pub enum ModelFuel {
    Gasoline,
    Diesel,
    Other,
}
