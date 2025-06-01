use super::super::empty_string_as_none;
use askama::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateClient {
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
#[template(path = "client_create.html")]
pub struct CreateClientTemplate;
