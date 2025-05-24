use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Constructor, Display, Serialize, Deserialize)]
pub struct Milage(f64);

impl TryFrom<String> for Milage {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.parse().map_err(|_| {
            "cannot parse string to mileage".to_string()
        })?))
    }
}
