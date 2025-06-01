use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Maintenance {
    filter_air: bool,
    filter_cabin: bool,
    filter_oil: bool,
    type_specific_maintenance: Option<MaintenanceType>,
}

impl Maintenance {
    pub fn new(
        filter_air: bool,
        filter_cabin: bool,
        filter_oil: bool,
        type_specific_maintenance: Option<MaintenanceType>,
    ) -> Self {
        Self {
            filter_air,
            filter_cabin,
            filter_oil,
            type_specific_maintenance,
        }
    }
    pub fn filter_air(&self) -> bool {
        self.filter_air
    }
    pub fn filter_cabin(&self) -> bool {
        self.filter_cabin
    }
    pub fn filter_oil(&self) -> bool {
        self.filter_oil
    }
    pub fn type_specific_maintenance(&self) -> Option<MaintenanceType> {
        self.type_specific_maintenance
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
pub enum MaintenanceType {
    FilterGasoil,
    SparkPlug,
}
