use super::super::common::car::*;

#[derive(Debug, Clone)]
pub struct Car {
    brand: Brand,
    cc: Cc,
    fuel: Fuel,
    model: Model,
    oil_quantity: OilQuantity,
    oil_type: OilType,
}
