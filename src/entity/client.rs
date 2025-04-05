use crate::repositoty::model::ModelClient;

use super::super::common::*;

#[derive(Debug, Clone)]
pub struct Client {
    name: String,
    address: Address,
    phone: Option<Phone>,
    email: Option<Email>,
}

impl From<ModelClient> for Client {
    fn from(client: ModelClient) -> Self {
        Self{
            name: format!("{} {}", client.first_name(), client.last_name()),
            address: Address::new(client.address()),
            phone: client.phone().map(|x| Phone::new(x)),
            email: client.email().map(|x| Email::new(x))
        }
    }
}