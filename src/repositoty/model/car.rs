use strum_macros::Display;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCar {
    id: RecordId,
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


impl ModelCar {
    pub fn id(&self) -> RecordId {
        self.id.clone()
    }
    pub fn client_id(&self) -> RecordId {
        self.client_id.clone()
    }
    pub fn brand(&self) -> String {
        self.brand.clone()
    }
    pub fn model(&self) -> String {
        self.model.clone()
    }
    pub fn cc(&self) -> String {
        self.cc.clone()
    }
    pub fn oil_type(&self) -> String {
        self.oil_type.clone()
    }
    pub fn oil_quantity(&self) -> String {
        self.oil_quantity.clone()
    }
    pub fn year(&self) -> u32 {
        self.year.clone()
    }
    pub fn fuel(&self) -> ModelFuel {
        self.fuel.clone()
    }
    pub fn created_at(&self) -> Datetime {
        self.created_at.clone()
    }
    pub fn updated_at(&self) -> Datetime {
        self.updated_at.clone()
    }
}
