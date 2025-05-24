use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::repositoty::model::ModelInterventionType;

use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InterventionType {
    Repair,
    Maintenance(Maintenance),
}

impl TryFrom<ModelInterventionType> for InterventionType {
    type Error = String;
    fn try_from(value: ModelInterventionType) -> Result<Self, Self::Error> {
        Ok(match value {
            ModelInterventionType::Repair => Self::Repair,
            ModelInterventionType::Maintenance(v) => Self::Maintenance(v.try_into()?),
        })
    }
}
