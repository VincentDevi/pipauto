use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Display, Serialize, Deserialize)]
pub struct Model(String);
