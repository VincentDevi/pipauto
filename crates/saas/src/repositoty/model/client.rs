use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

use crate::common::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelClient {
    id: RecordId,
    first_name: String,
    last_name: String,
    address: AddressModel,
    phone: Option<String>,
    email: Option<String>,
    created_at: Datetime,
    updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressModel {
    street: String,
    number: String,
    postal: String,
    country: String,
}

impl AddressModel {
    pub fn street(&self) -> String {
        self.street.clone()
    }
    pub fn postal(&self) -> String {
        self.postal.clone()
    }
    pub fn number(&self) -> String {
        self.number.clone()
    }
    pub fn country(&self) -> String {
        self.country.clone()
    }
}

impl From<Address> for AddressModel {
    fn from(value: Address) -> Self {
        Self {
            street: value.street(),
            country: value.country(),
            postal: value.postal(),
            number: value.number(),
        }
    }
}

impl ModelClient {
    pub fn id(&self) -> RecordId {
        self.id.clone()
    }
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }
    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
    pub fn address(&self) -> AddressModel {
        self.address.clone()
    }
    pub fn phone(&self) -> Option<String> {
        self.phone.clone()
    }
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
    pub fn created_at(&self) -> Datetime {
        self.created_at.clone()
    }
    pub fn updated_at(&self) -> Datetime {
        self.updated_at.clone()
    }
}
