mod intervention_type;
mod maintenance;

use super::error::*;
use super::{car::*, std::*};
pub use intervention_type::*;
pub use maintenance::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Intervention {
    intervention_date: Date,
    price: Price,
    mileage: Milage,
    remarks: Vec<String>,
    intervention_type: InterventionType,
}

impl Intervention {
    pub fn intervention_date(&self) -> &Date {
        &self.intervention_date
    }
    pub fn price(&self) -> &Price {
        &self.price
    }
    pub fn mileage(&self) -> &Milage {
        &self.mileage
    }
    pub fn remarks(&self) -> &[String] {
        &self.remarks
    }
    pub fn intervention_type(&self) -> &InterventionType {
        &self.intervention_type
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterventionBuilder {
    intervention_date: Option<Date>,
    price: Option<Price>,
    mileage: Option<Milage>,
    remarks: Option<Vec<String>>,
    intervention_type: Option<InterventionType>,
}

impl InterventionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn price(&mut self, price: Price) -> &mut Self {
        self.price = Some(price);
        self
    }
    pub fn intervention_date(&mut self, date: Date) -> &mut Self {
        self.intervention_date = Some(date);
        self
    }
    pub fn mileage(&mut self, mileage: Milage) -> &mut Self {
        self.mileage = Some(mileage);
        self
    }
    pub fn remarks(&mut self, remarks: Vec<String>) -> &mut Self {
        self.remarks = Some(remarks);
        self
    }
    pub fn intervention_type(&mut self, intervention_type: InterventionType) -> &mut Self {
        self.intervention_type = Some(intervention_type);
        self
    }
    pub fn build(&mut self) -> Result<Intervention, Error> {
        Ok(Intervention {
            intervention_date: self.intervention_date.ok_or(Error::Parsing(Arc::from(
                "missing date while parsing an intervention",
            )))?,
            price: self.price.clone().ok_or(Error::Parsing(Arc::from(
                "missing price while parsing an intervention",
            )))?,
            mileage: self.mileage.ok_or(Error::Parsing(Arc::from(
                "missing mileage while parsing an intervention",
            )))?,
            remarks: self.remarks.clone().ok_or(Error::Parsing(Arc::from(
                "missings remarks while parsing an intervention",
            )))?,
            intervention_type: self.intervention_type.ok_or(Error::Parsing(Arc::from(
                "missing type while parsing an intervention",
            )))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecificInterventionWithCar {
    intervention_date: Date,
    price: Price,
    mileage: Milage,
    remarks: Vec<String>,
    intervention_type: InterventionType,
    car: SpecificCarForIntervention,
}
impl SpecificInterventionWithCar {
    pub fn intervention_date(&self) -> &Date {
        &self.intervention_date
    }
    pub fn price(&self) -> &Price {
        &self.price
    }
    pub fn mileage(&self) -> &Milage {
        &self.mileage
    }
    pub fn remarks(&self) -> &[String] {
        &self.remarks
    }
    pub fn intervention_type(&self) -> &InterventionType {
        &self.intervention_type
    }
    pub fn car(&self) -> &SpecificCarForIntervention {
        &self.car
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpecificInterventionWithCarBuilder {
    intervention_date: Option<Date>,
    price: Option<Price>,
    mileage: Option<Milage>,
    remarks: Option<Vec<String>>,
    intervention_type: Option<InterventionType>,
    car: Option<SpecificCarForIntervention>,
}

impl SpecificInterventionWithCarBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn price(&mut self, price: Price) -> &mut Self {
        self.price = Some(price);
        self
    }
    pub fn intervention_date(&mut self, date: Date) -> &mut Self {
        self.intervention_date = Some(date);
        self
    }
    pub fn mileage(&mut self, mileage: Milage) -> &mut Self {
        self.mileage = Some(mileage);
        self
    }
    pub fn remarks(&mut self, remarks: Vec<String>) -> &mut Self {
        self.remarks = Some(remarks);
        self
    }
    pub fn intervention_type(&mut self, intervention_type: InterventionType) -> &mut Self {
        self.intervention_type = Some(intervention_type);
        self
    }
    pub fn car(&mut self, car: SpecificCarForIntervention) -> &mut Self {
        self.car = Some(car);
        self
    }
    pub fn build(&mut self) -> Result<SpecificInterventionWithCar, Error> {
        Ok(SpecificInterventionWithCar {
            intervention_date: self.intervention_date.ok_or(Error::Parsing(Arc::from(
                "missing date while parsing an intervention",
            )))?,
            price: self.price.clone().ok_or(Error::Parsing(Arc::from(
                "missing price while parsing an intervention",
            )))?,
            mileage: self.mileage.ok_or(Error::Parsing(Arc::from(
                "missing mileage while parsing an intervention",
            )))?,
            remarks: self.remarks.clone().ok_or(Error::Parsing(Arc::from(
                "missings remarks while parsing an intervention",
            )))?,
            intervention_type: self.intervention_type.ok_or(Error::Parsing(Arc::from(
                "missing type while parsing an intervention",
            )))?,
            car: self.car.clone().ok_or(Error::Parsing(Arc::from(
                "missing car while parsing an intervention",
            )))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecificCarForIntervention {
    id: String,
    brand: Brand,
    model: Model,
}

impl SpecificCarForIntervention {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn brand(&self) -> &Brand {
        &self.brand
    }
    pub fn model(&self) -> &Model {
        &self.model
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpecificCarForInterventionBuilder {
    id: Option<String>,
    brand: Option<Brand>,
    model: Option<Model>,
}

impl SpecificCarForInterventionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
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
    pub fn build(&mut self) -> Result<SpecificCarForIntervention, Error> {
        Ok(SpecificCarForIntervention {
            id: self
                .id
                .clone()
                .ok_or(Error::Parsing(Arc::from("missing id while parsing a car")))?,
            brand: self
                .brand
                .clone()
                .ok_or(Error::Parsing(Arc::from("missing brand while parsing car")))?,
            model: self.model.clone().ok_or(Error::Parsing(Arc::from(
                "missing model while parsing a car",
            )))?,
        })
    }
}
