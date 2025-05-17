use serde::{Deserialize, Serialize};

use crate::repositoty::model::ModelMaintenance;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Maintenance {
    filter_air: bool,
    filter_cabin: bool,
    filter_oil: bool,
    type_specific_maintenance: Option<MaintenanceType>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MaintenanceType {
    FilterGasoil,
    SparkPlug,
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
        Ok(Self {
            filter_air: value.filter_air(),
            filter_cabin: value.filter_cabin(),
            filter_oil: value.filter_oil(),
            type_specific_maintenance,
        })
    }
}
