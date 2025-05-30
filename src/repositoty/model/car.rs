use serde::{Deserialize, Serialize};
use strum_macros::Display;
use surrealdb::{Datetime, RecordId};

use super::ModelIntervertion;

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
    interventions: Vec<ModelIntervertion>,
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
        self.year
    }
    pub fn fuel(&self) -> ModelFuel {
        self.fuel
    }
    pub fn created_at(&self) -> Datetime {
        self.created_at.clone()
    }
    pub fn updated_at(&self) -> Datetime {
        self.updated_at.clone()
    }
    pub fn interventions(&self) -> Vec<ModelIntervertion> {
        self.interventions.clone()
    }
}
