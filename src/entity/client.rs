use serde::{Deserialize, Serialize};

use crate::repositoty::model::ModelClient;

use super::super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    id: ClientRecordId,
    first_name: String,
    last_name: String,
    full_name: String,
    address: Address,
    phone: Option<Phone>,
    email: Option<Email>,
}

impl TryFrom<ModelClient> for Client {
    type Error = String;
    fn try_from(client: ModelClient) -> Result<Self, Self::Error> {
        Ok(Self {
            id: client.id().try_into()?,
            first_name: client.first_name(),
            last_name: client.last_name(),
            full_name: format!("{} {}", client.first_name(), client.last_name()),
            address: Address::new(client.address()),
            phone: client.phone().map(Phone::new),
            email: client.email().map(Email::new),
        })
    }
}

impl Client {
    pub fn id(&self) -> ClientRecordId {
        self.id.clone()
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn full_name(&self) -> String {
        self.full_name.clone()
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
