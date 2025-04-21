use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::{
    handler::client::CreateClient,
    repositoty::{RepositoryError, model::ModelClient},
};

use super::super::Repository;

impl Repository {
    pub async fn update_client(
        &self,
        client_id: RecordId,
        form_data: CreateClientEntity,
    ) -> Result<(), RepositoryError> {
        let create_client_model: CreateClientModel = form_data.into();
        let _record: Vec<ModelClient> = self
            .db
            .update(client_id.to_string())
            .content(create_client_model)
            .await?;
        Ok(())
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
