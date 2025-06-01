use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelClient;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn get_client(&self, id: &str) -> Result<Client, RepositoryError> {
        let query = format!("SELECT * FROM client:{};", id);
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelClient> = response.take(0)?;
        let client = result
            .first()
            .cloned()
            .ok_or(RepositoryError::DatabaseError)?;
        client.try_into().map_err(RepositoryError::ParsingError)
    }
}
