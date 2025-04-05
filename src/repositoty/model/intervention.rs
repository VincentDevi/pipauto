use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone)]
pub struct ModelIntervertion {
    car_id: RecordId,
    price: f64,
    mileage: f64,
    remarks: Vec<String>,
    intervention_date: Datetime,
    created_at: Datetime,
    updated_at: Datetime,
}

#[derive(Debug, Clone)]
pub enum ModelInterventionType {
    Repair,
    Maintenance(ModelMaintenance),
}

#[derive(Debug, Clone)]
pub struct ModelMaintenance {
    filter_air: bool,
    filter_cabin: bool,
    filter_gasoil: bool,
    filter_oil: bool,
    spark_plug: bool,
}
