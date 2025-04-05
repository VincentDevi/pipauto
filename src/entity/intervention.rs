use super::super::common::{car::*, intervention::*};
use super::Car;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Intervention {
    intervention_date: NaiveDateTime,
    car: Car,
    price: Price,
    mileage: Milage,
    remarks: Vec<String>,
    intervention_type: InterventionType,
}
