use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelClient;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn get_client(&self, id: String) -> Result<Client, RepositoryError> {
        let query = format!("SELECT * FROM client:{};", id);
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelClient> = response.take(0)?;
        let client = result
            .get(0)
            .cloned()
            .ok_or(RepositoryError::DatabaseError)?;
        Ok(client.into())
    }
}
