use serde::{Deserialize, Serialize};

use crate::repositoty::model::ModelClient;

use super::super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    id: String,
    name: String,
    address: Address,
    phone: Option<Phone>,
    email: Option<Email>,
}

impl From<ModelClient> for Client {
    fn from(client: ModelClient) -> Self {
        Self {
            id: client.id().to_string(),
            name: format!("{} {}", client.first_name(), client.last_name()),
            address: Address::new(client.address()),
            phone: client.phone().map(|x| Phone::new(x)),
            email: client.email().map(|x| Email::new(x)),
        }
    }
}

