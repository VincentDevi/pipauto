use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Serialize, Display, Deserialize)]
pub struct Cc(String);
