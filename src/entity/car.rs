use serde::{Deserialize, Serialize};

use super::super::common::car::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    car_id: String,
    client_id: String,
    brand: Brand,
    model: Model,
    cc: Cc,
    fuel: Fuel,
    oil_quantity: OilQuantity,
    oil_type: OilType,
}
