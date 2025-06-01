use strum_macros::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize)]
pub enum Fuel {
    Gasoline,
    Diesel,
    Other,
}
