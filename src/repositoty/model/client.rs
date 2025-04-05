use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelClient {
    id: RecordId,
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
    created_at: Datetime,
    updated_at: Datetime,
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
    pub fn address(&self) -> String {
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
