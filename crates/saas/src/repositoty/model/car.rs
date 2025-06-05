use std::sync::Arc;

use super::ModelIntervertion;
use common::{car::*, error::Error, std::*};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
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

impl From<ModelFuel> for Fuel {
    fn from(value: ModelFuel) -> Self {
        match value {
            ModelFuel::Gasoline => Self::Gasoline,
            ModelFuel::Diesel => Self::Diesel,
            ModelFuel::Other => Self::Other,
        }
    }
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

impl TryFrom<ModelCar> for Car {
    type Error = Error;
    fn try_from(value: ModelCar) -> Result<Self, Self::Error> {
        CarBuilder::new()
            .car(value.id.to_string())
            .client(value.client_id.to_string())
            .brand(Brand::new(value.brand))
            .model(Model::new(value.model))
            .cc(Cc::new(value.cc))
            .fuel(value.fuel.into())
            .oil_quantity(OilQuantity::new(value.oil_quantity))
            .oil_type(OilType::new(value.oil_type))
            .year(Year::new(value.year))
            .interventions(
                value
                    .interventions
                    .into_iter()
                    .map(|x| x.try_into())
                    .collect()?,
            )
            .build()
            .map_err(|e| Error::Parsing(Arc::from("a")))
    }
}
