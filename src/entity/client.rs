use super::super::common::*;

#[derive(Debug, Clone)]
pub struct Client {
    name: String,
    adress: Address,
    phone: Option<Phone>,
    email: Option<Email>,
}
