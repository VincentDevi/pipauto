use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Serialize, Deserialize)]
pub struct Phone(String);
