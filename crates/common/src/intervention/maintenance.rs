use std::sync::Arc;

use super::super::error::*;
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

#[derive(Debug, Clone, Copy, Default)]
pub struct MaintenanceBuilder {
    filter_air: Option<bool>,
    filter_cabin: Option<bool>,
    filter_oil: Option<bool>,
    type_specific_maintenance: Option<MaintenanceType>,
}

impl MaintenanceBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn filter_air(&mut self, filter_air: bool) -> &mut Self {
        self.filter_air = Some(filter_air);
        self
    }
    pub fn filter_cabin(&mut self, filter_cabin: bool) -> &mut Self {
        self.filter_cabin = Some(filter_cabin);
        self
    }
    pub fn filter_oil(&mut self, filter_oil: bool) -> &mut Self {
        self.filter_oil = Some(filter_oil);
        self
    }
    pub fn type_specific_maintenance(
        &mut self,
        type_specific_maintenance: Option<MaintenanceType>,
    ) -> &mut Self {
        self.type_specific_maintenance = type_specific_maintenance;
        self
    }
    pub fn build(&mut self) -> Result<Maintenance, Error> {
        Ok(Maintenance {
            filter_air: self.filter_air.ok_or(Error::Parsing(Arc::from(
                "missing filter air while creating a maintenance",
            )))?,
            filter_cabin: self.filter_cabin.ok_or(Error::Parsing(Arc::from(
                "missing filter cabin while creating a maintenance",
            )))?,
            filter_oil: self.filter_oil.ok_or(Error::Parsing(Arc::from(
                "missing filter oil while creating a maintenance",
            )))?,
            type_specific_maintenance: self.type_specific_maintenance,
        })
    }
}
