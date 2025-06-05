mod brand;
mod cc;
mod fuel;
mod mileage;
mod model;
mod oil_quantity;
mod oil_type;
mod price;

use std::sync::Arc;

pub use brand::*;
pub use cc::*;
pub use fuel::*;
pub use mileage::*;
pub use model::*;
pub use oil_quantity::*;
pub use oil_type::*;
pub use price::*;

use super::error::Error;
use super::{intervention::*, std::*};
use serde::{Deserialize, Serialize};

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

impl Car {
    pub fn cc(&self) -> Cc {
        self.cc.clone()
    }
    pub fn model(&self) -> Model {
        self.model.clone()
    }
    pub fn intervention(&self) -> Vec<Intervention> {
        self.interventions.clone()
    }
    pub fn brand(&self) -> Brand {
        self.brand.clone()
    }
    pub fn fuel(&self) -> Fuel {
        self.fuel
    }
    pub fn oil_type(&self) -> OilType {
        self.oil_type.clone()
    }
    pub fn oil_quantity(&self) -> OilQuantity {
        self.oil_quantity.clone()
    }
    pub fn year(&self) -> Year {
        self.year
    }
}

#[derive(Debug, Clone, Default)]
pub struct CarBuilder {
    car_id: Option<String>,
    client_id: Option<String>,
    brand: Option<Brand>,
    model: Option<Model>,
    cc: Option<Cc>,
    fuel: Option<Fuel>,
    oil_quantity: Option<OilQuantity>,
    oil_type: Option<OilType>,
    year: Option<Year>,
    interventions: Option<Vec<Intervention>>,
}

impl CarBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn car(&mut self, car: String) -> &mut Self {
        self.car_id = Some(car);
        self
    }
    pub fn client(&mut self, client: String) -> &mut Self {
        self.client_id = Some(client);
        self
    }
    pub fn brand(&mut self, brand: Brand) -> &mut Self {
        self.brand = Some(brand);
        self
    }
    pub fn model(&mut self, model: Model) -> &mut Self {
        self.model = Some(model);
        self
    }
    pub fn cc(&mut self, cc: Cc) -> &mut Self {
        self.cc = Some(cc);
        self
    }
    pub fn fuel(&mut self, fuel: Fuel) -> &mut Self {
        self.fuel = Some(fuel);
        self
    }
    pub fn oil_quantity(&mut self, oil_quantity: OilQuantity) -> &mut Self {
        self.oil_quantity = Some(oil_quantity);
        self
    }
    pub fn oil_type(&mut self, oil_type: OilType) -> &mut Self {
        self.oil_type = Some(oil_type);
        self
    }
    pub fn year(&mut self, year: Year) -> &mut Self {
        self.year = Some(year);
        self
    }
    pub fn interventions(&mut self, interventions: Vec<Intervention>) -> &mut Self {
        self.interventions = Some(interventions);
        self
    }
    pub fn build(&mut self) -> Result<Car, Error> {
        Ok(Car {
            car_id: self.car_id.clone().ok_or(Error::Parsing(Arc::from(
                "missing car id while creating a car",
            )))?,
            client_id: self.client_id.clone().ok_or(Error::Parsing(Arc::from(
                "missing client while creating a car",
            )))?,
            brand: self.brand.clone().ok_or(Error::Parsing(Arc::from(
                "missing brand while creating a car",
            )))?,
            model: self.model.clone().ok_or(Error::Parsing(Arc::from(
                "missing model while creating a car",
            )))?,
            cc: self
                .cc
                .clone()
                .ok_or(Error::Parsing(Arc::from("missing cc while creating a car")))?,
            fuel: self.fuel.ok_or(Error::Parsing(Arc::from(
                "missing fuel while creating a car",
            )))?,
            oil_quantity: self.oil_quantity.clone().ok_or(Error::Parsing(Arc::from(
                "missing oil quantity while creating a car",
            )))?,
            oil_type: self.oil_type.clone().ok_or(Error::Parsing(Arc::from(
                "missing oil type while creating a car",
            )))?,
            year: self.year.ok_or(Error::Parsing(Arc::from(
                "missing year while creating a car",
            )))?,
            interventions: self.interventions.clone().ok_or(Error::Parsing(Arc::from(
                "missing interventions while creating a car",
            )))?,
        })
    }
}
