use derive_more::{Constructor, Display};

#[derive(Debug, Clone, Constructor, Display)]
pub struct OilQuantity(f64);
