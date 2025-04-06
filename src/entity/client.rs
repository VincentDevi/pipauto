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
            phone: client.phone().map(Phone::new),
            email: client.email().map(Email::new),
        }
    }
}

impl Client {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn phone(&self) -> Option<Phone> {
        self.phone.clone()
    }

    pub fn email(&self) -> Option<Email> {
        self.email.clone()
    }
}
