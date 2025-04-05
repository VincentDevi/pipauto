use super::super::common::{car::*, intervention::*};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Intervention {
    intervention_date: NaiveDateTime,
    price: Price,
    mileage: Milage,
    remarks: Vec<String>,
    intervention_type: InterventionType,
}
