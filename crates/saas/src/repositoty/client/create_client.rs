use crate::repositoty::{RepositoryError, model::ModelClient};
use common::{client::*, std::*};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use templating::client::CreateClient;

use super::super::Repository;

impl Repository {
    pub async fn create_client(
        &self,
        form_data: CreateClientEntity,
    ) -> Result<Client, RepositoryError> {
        let create_client_model: CreateClientModel = form_data.into();
        let record: ModelClient = self
            .db
            .create("client")
            .content(create_client_model)
            .await?
            .ok_or(RepositoryError::DatabaseError)?;
        Ok(ClientBuilder::new()
            .first_name(record.first_name())
            .last_name(record.last_name())
            .full_name(format!("{} {}", record.first_name(), record.last_name()))
            .phone(record.phone().map(Phone::new))
            .email(record.email().map(Email::new))
            .address(record.address().into())
            .build()?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateClientEntity {
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
    car: Option<RecordId>,
}

impl From<CreateClient> for CreateClientEntity {
    fn from(value: CreateClient) -> Self {
        Self {
            first_name: value.first_name,
            last_name: value.last_name,
            address: value.address,
            phone: value.phone,
            email: value.email,
            car: None, // Need to refactor this and create my RecordId Structs
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateClientModel {
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
    //    car: Option<RecordId>,
}

impl From<CreateClientEntity> for CreateClientModel {
    fn from(value: CreateClientEntity) -> Self {
        Self {
            first_name: value.first_name,
            last_name: value.last_name,
            address: value.address,
            phone: value.phone,
            email: value.email,
            //            car: value.car,
        }
    }
}
