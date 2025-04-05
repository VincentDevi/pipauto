use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Display, Deserialize, Serialize)]
pub struct OilQuantity(String);
