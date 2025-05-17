use std::str::FromStr;

use crate::repositoty::model::ModelIntervertion;

use super::super::common::{car::*, intervention::*};
use chrono::NaiveDateTime;
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
        Ok(Self {
            intervention_type: value.intervention_type().try_into()?,
            price: value.price().try_into()?,
            mileage: Milage::new(value.mileage()),
            remarks: Vec::default(), // this should be fixed later on
            intervention_date: NaiveDateTime::from_str(&value.intervention_date().to_string())
                .map_err(|x| x.to_string())?,
        })
    }
}
