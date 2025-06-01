mod car;
mod car_owner_history;
mod client;
mod intervention;

pub use car::*;
pub use car_owner_history::*;
pub use client::*;
use common::{
    intervention::{InterventionType, Maintenance, MaintenanceType},
    std::Address,
};
pub use intervention::*;

impl From<AddressModel> for Address {
    fn from(value: AddressModel) -> Self {
        Self::new(
            value.street(),
            value.number(),
            value.country(),
            value.postal(),
        )
    }
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

impl TryFrom<ModelMaintenance> for Maintenance {
    type Error = String;
    fn try_from(value: ModelMaintenance) -> Result<Self, Self::Error> {
        let type_specific_maintenance = match (value.spark_pulg(), value.filter_gasoil()) {
            (true, false) => Ok(Some(MaintenanceType::SparkPlug)),
            (false, true) => Ok(Some(MaintenanceType::FilterGasoil)),
            (false, false) => Ok(None),
            _ => Err("error in our database".to_string()),
        }?;
        Ok(Self::new(
            value.filter_air(),
            value.filter_cabin(),
            value.filter_oil(),
            type_specific_maintenance,
        ))
    }
}
