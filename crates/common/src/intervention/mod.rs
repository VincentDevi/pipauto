mod intervention_type;
mod maintenance;

use super::{car::*, std::*};
pub use intervention_type::*;
pub use maintenance::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecificCarForIntervention {
    pub id: String,
    pub brand: String,
    pub model: String,
}

impl SpecificCarForIntervention {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn brand(&self) -> &str {
        &self.brand
    }
    pub fn model(&self) -> &str {
        &self.model
    }
}
