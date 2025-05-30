use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Serialize, Deserialize, Display)]
pub struct Phone(String);
