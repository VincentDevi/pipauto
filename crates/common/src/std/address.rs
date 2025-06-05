use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    country: String,
    street: String,
    number: String,
    postal: String,
}

impl Address {
    pub fn new(street: String, number: String, country: String, postal: String) -> Self {
        Self {
            country,
            street,
            number,
            postal,
        }
    }
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

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {}, {}",
            self.street, self.number, self.postal, self.country
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct AddressBuilder<'a> {
    country: Option<&'a str>,
    street: Option<&'a str>,
    number: Option<&'a str>,
    postal: Option<&'a str>,
}

impl<'a> AddressBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(&mut self, value: &'a str) -> &mut Self {
        self.country = Some(value);
        self
    }

    pub fn street(&mut self, value: &'a str) -> &mut Self {
        self.street = Some(value);
        self
    }

    pub fn number(&mut self, value: &'a str) -> &mut Self {
        self.number = Some(value);
        self
    }

    pub fn postal(&mut self, value: &'a str) -> &mut Self {
        self.postal = Some(value);
        self
    }

    pub fn build(&mut self) -> Result<Address, String> {
        Ok(Address {
            country: self
                .country
                .ok_or("missing country".to_string())?
                .to_string(),
            street: self.street.ok_or("missing street")?.to_string(),
            number: self.number.ok_or("cant convert".to_string())?.to_string(),
            postal: self.postal.ok_or("cant convert".to_string())?.to_string(),
        })
    }
}
