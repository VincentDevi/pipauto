use crate::repositoty::{ModelIntervertionWithCar, model::ModelIntervertion};
use chrono::{DateTime, NaiveDateTime};
use common::{car::*, intervention::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Intervention {
    intervention_date: NaiveDateTime,
    price: Price,
    mileage: Milage,
    remarks: Vec<String>,
    intervention_type: InterventionType,
}

impl TryFrom<ModelIntervertion> for Intervention {
    type Error = String;
    fn try_from(value: ModelIntervertion) -> Result<Self, Self::Error> {
        let date = value.intervention_date().to_string();
        let s = date.trim_start_matches("d'").trim_end_matches('\'');
        let date_time =
            DateTime::parse_from_rfc3339(s).map_err(|_| "error with date parsing".to_string());

        Ok(Self {
            intervention_type: value.intervention_type().try_into()?,
            price: value.price().try_into()?,
            mileage: value.mileage().try_into()?,
            remarks: Vec::default(), // this should be fixed later on
            intervention_date: date_time?.naive_utc(),
        })
    }
}

impl Intervention {
    pub fn intervention_date(&self) -> &NaiveDateTime {
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
    intervention_date: NaiveDateTime,
    price: Price,
    mileage: Milage,
    remarks: Vec<String>,
    intervention_type: InterventionType,
    car: SpecificCarForIntervention,
}
impl SpecificInterventionWithCar {
    pub fn intervention_date(&self) -> &NaiveDateTime {
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

impl TryFrom<ModelIntervertionWithCar> for SpecificInterventionWithCar {
    type Error = String;
    fn try_from(value: ModelIntervertionWithCar) -> Result<Self, Self::Error> {
        let date = value.intervention_date().to_string();
        let s = date.trim_start_matches("d'").trim_end_matches('\'');
        let date_time =
            DateTime::parse_from_rfc3339(s).map_err(|_| "error with date parsing".to_string());

        Ok(Self {
            intervention_type: value.intervention_type().try_into()?,
            price: value.price().try_into()?,
            mileage: value.mileage().try_into()?,
            remarks: Vec::default(), // this should be fixed later on
            intervention_date: date_time?.naive_utc(),
            car: SpecificCarForIntervention {
                id: value.car_id.id.to_string(),
                brand: value.car_id.brand.to_string(),
                model: value.car_id.model.to_string(),
            },
        })
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
