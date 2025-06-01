mod brand;
mod cc;
mod fuel;
mod mileage;
mod model;
mod oil_quantity;
mod oil_type;
mod price;

pub use brand::*;
pub use cc::*;
pub use fuel::*;
pub use mileage::*;
pub use model::*;
pub use oil_quantity::*;
pub use oil_type::*;
pub use price::*;

use super::{intervention::*, std::*};
use serde::{Deserialize, Serialize};

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
    year: Year,
    interventions: Vec<Intervention>,
}
