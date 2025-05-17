use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelIntervertion {
    id: RecordId,
    client: RecordId,
    car_id: RecordId,
    price: f64,
    mileage: f64,
    remarks: Vec<String>,
    intervention_date: Datetime,
    intervention_type: ModelInterventionType,
    created_at: Datetime,
    updated_at: Datetime,
}

impl ModelIntervertion {
    pub fn intervention_date(&self) -> Datetime {
        self.intervention_date.clone()
    }
    pub fn intervention_type(&self) -> ModelInterventionType {
        self.intervention_type.clone()
    }
    pub fn created_at(&self) -> Datetime {
        self.created_at.clone()
    }
    pub fn id(&self) -> RecordId {
        self.id.clone()
    }
    pub fn client(&self) -> RecordId {
        self.client.clone()
    }
    pub fn car_id(&self) -> RecordId {
        self.car_id.clone()
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn mileage(&self) -> f64 {
        self.mileage
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ModelInterventionType {
    Repair,
    Maintenance(ModelMaintenance),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelMaintenance {
    filter_air: bool,
    filter_cabin: bool,
    filter_gasoil: bool,
    filter_oil: bool,
    spark_plug: bool,
}

impl ModelMaintenance {
    pub fn filter_air(&self) -> bool {
        self.filter_air
    }
    pub fn filter_cabin(&self) -> bool {
        self.filter_cabin
    }
    pub fn filter_gasoil(&self) -> bool {
        self.filter_gasoil
    }
    pub fn filter_oil(&self) -> bool {
        self.filter_oil
    }
    pub fn spark_pulg(&self) -> bool {
        self.spark_plug
    }
}
