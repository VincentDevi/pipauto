use serde::{Deserialize, Serialize};
use surrealdb::Datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelClient {
    first_name: String,
    last_name: String,
    adress: String,
    phone: Option<String>,
    email: Option<String>,
    created_at: Datetime,
    updated_at: Datetime,
}
