use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::{
    entity::Client,
    handler::client::CreateClient,
    repositoty::{RepositoryError, model::ModelClient},
};

use super::super::Repository;

impl Repository {
    pub async fn create_client(
        &self,
        form_data: CreateClientEntity,
    ) -> Result<Client, RepositoryError> {
        let create_client_model: CreateClientModel = form_data.into();
        let record: Option<ModelClient> = self
            .db
            .create("client")
            .content(create_client_model)
            .await?;
        Ok(record
            .ok_or(RepositoryError::DatabaseError)?
            .try_into()
            .map_err(RepositoryError::ParsingError)?)
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
