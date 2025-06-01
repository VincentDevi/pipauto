use super::std::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    id: String,
    first_name: String,
    last_name: String,
    full_name: String,
    address: Address,
    phone: Option<Phone>,
    email: Option<Email>,
}

impl Client {
    pub fn id(&self) -> String {
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
