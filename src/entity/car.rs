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
    interventions: Intervention,
}

impl From<ModelCar> for Car {
    fn from(car: ModelCar) -> Self {
        Self {
            car_id: car.id().to_string(),
            client_id: car.client_id().to_string(),
            brand: Brand::new(car.brand()),
            model: Model::new(car.model()),
            cc: Cc::new(car.cc()),
            fuel: match car.fuel() {
                ModelFuel::Gasoline => Fuel::Gasoline,
                ModelFuel::Diesel => Fuel::Diesel,
                ModelFuel::Other => Fuel::Other,
            },
            oil_quantity: OilQuantity::new(car.oil_quantity()),
            oil_type: OilType::new(car.oil_type()),
            year: Year::new(car.year()),
            interventions: car.interventions().into(),
        }
    }
}

impl Car {
    pub fn cc(&self) -> &Cc {
        &self.cc
    }
    pub fn brand(&self) -> &Brand {
        &self.brand
    }
    pub fn fuel(&self) -> &Fuel {
        &self.fuel
    }
    pub fn oil_quantity(&self) -> &OilQuantity {
        &self.oil_quantity
    }
    pub fn oil_qtype(&self) -> &OilType {
        &self.oil_type
    }
    pub fn year(&self) -> &Year {
        &self.year
    }
}
