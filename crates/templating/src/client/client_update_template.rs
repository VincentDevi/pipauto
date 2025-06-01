use super::super::empty_string_as_none;
use askama::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateClient {
    pub first_name: String,
    pub last_name: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    pub address: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub car: Option<String>,
}

#[derive(Template)]
#[template(path = "client_update.html")]
pub struct UpdateClientTemplate {
    id: String,
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
}
