use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InterventionType {
    Repair,
    Maintenance(Maintenance),
}

impl Display for InterventionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Repair => write!(f, "Repair"),
            Self::Maintenance(_) => write!(f, "Maintenance"),
        }
    }
}
