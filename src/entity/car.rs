use serde::{Deserialize, Serialize};

use crate::{
    common::Year,
    repositoty::model::{ModelCar, ModelFuel},
};

use super::{super::common::car::*, Intervention};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    car_id: String,
    client_id: String,
    brand: Brand,
    model: Model,
    cc: Cc,
    fuel: Fuel,
    oil_quantity: OilQuantity,
    oil_type: OilType,
    year: Year,
    interventions: Vec<Intervention>,
}

impl TryFrom<ModelCar> for Car {
    type Error = String;
    fn try_from(value: ModelCar) -> Result<Self, Self::Error> {
        Ok(Self {
            car_id: value.id().to_string(),
            client_id: value.client_id().to_string(),
            brand: Brand::new(value.brand()),
            model: Model::new(value.model()),
            cc: Cc::new(value.cc()),
            fuel: match value.fuel() {
                ModelFuel::Gasoline => Fuel::Gasoline,
                ModelFuel::Diesel => Fuel::Diesel,
                ModelFuel::Other => Fuel::Other,
            },
            oil_quantity: OilQuantity::new(value.oil_quantity()),
            oil_type: OilType::new(value.oil_type()),
            year: Year::new(value.year()),
            interventions: value
                .interventions()
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<Intervention>, _>>()?,
        })
    }
}

impl Car {
    pub fn cc(&self) -> &Cc {
        &self.cc
    }
    pub fn brand(&self) -> &Brand {
        &self.brand
    }
    pub fn model(&self) -> &Model {
        &self.model
    }
    pub fn fuel(&self) -> &Fuel {
        &self.fuel
    }
    pub fn oil_quantity(&self) -> &OilQuantity {
        &self.oil_quantity
    }
    pub fn oil_type(&self) -> &OilType {
        &self.oil_type
    }
    pub fn year(&self) -> &Year {
        &self.year
    }
    pub fn intervention(&self) -> &[Intervention] {
        &self.interventions
    }
}
