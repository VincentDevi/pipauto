use super::error::*;
use super::std::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[derive(Debug, Clone, Default)]
pub struct ClientBuilder {
    id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    full_name: Option<String>,
    address: Option<Address>,
    phone: Option<Phone>,
    email: Option<Email>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn first_name(&mut self, first_name: String) -> &mut Self {
        self.first_name = Some(first_name);
        self
    }
    pub fn last_name(&mut self, last_name: String) -> &mut Self {
        self.last_name = Some(last_name);
        self
    }
    pub fn full_name(&mut self, full_name: String) -> &mut Self {
        self.full_name = Some(full_name);
        self
    }
    pub fn address(&mut self, address: Address) -> &mut Self {
        self.address = Some(address);
        self
    }
    pub fn phone(&mut self, phone: Option<Phone>) -> &mut Self {
        self.phone = phone;
        self
    }
    pub fn email(&mut self, email: Option<Email>) -> &mut Self {
        self.email = email;
        self
    }
    pub fn build(&mut self) -> Result<Client, Error> {
        Ok(Client {
            id: self.id.clone().ok_or(Error::Parsing(Arc::from(
                "missing id while creating a client",
            )))?,
            first_name: self.first_name.clone().ok_or(Error::Parsing(Arc::from(
                "missing first name while creating a client",
            )))?,
            last_name: self.last_name.clone().ok_or(Error::Parsing(Arc::from(
                "missing last name while creating a client",
            )))?,
            full_name: self.full_name.clone().ok_or(Error::Parsing(Arc::from(
                "missing full name while creating a client",
            )))?,
            address: self.address.clone().ok_or(Error::Parsing(Arc::from(
                "missing address while creating a client",
            )))?,
            phone: self.phone.clone(),
            email: self.email.clone(),
        })
    }
}
