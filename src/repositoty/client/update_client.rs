use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, opt::PatchOp};

use crate::{
    handler::client::UpdateClient,
    repositoty::{RepositoryError, model::ModelClient},
};

use super::super::Repository;

impl Repository {
    pub async fn update_client(
        &self,
        client_id: RecordId,
        form_data: UpdateClient,
    ) -> Result<(), RepositoryError> {
        let update_client_model: UpdateClientEntity = form_data.into();
        println!("test test : {}", client_id);
        let record: Option<ModelClient> = self
            .db
            .update(client_id)
            .patch(PatchOp::replace(
                "first_name",
                update_client_model.first_name,
            ))
            .patch(PatchOp::replace("last_name", update_client_model.last_name))
            .patch(PatchOp::replace("address", update_client_model.address))
            .patch(PatchOp::replace("phone", update_client_model.phone))
            .patch(PatchOp::replace("email", update_client_model.email))
            .await?;
        println!("{:?}", record);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateClientEntity {
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
    car: Option<RecordId>,
}

impl From<UpdateClient> for UpdateClientEntity {
    fn from(value: UpdateClient) -> Self {
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
pub struct UpdateClientModel {
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
    //    car: Option<RecordId>,
}

impl From<UpdateClientEntity> for UpdateClientModel {
    fn from(value: UpdateClientEntity) -> Self {
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
