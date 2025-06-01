use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Constructor, Display, Serialize, Deserialize)]
pub struct Year(u32);
