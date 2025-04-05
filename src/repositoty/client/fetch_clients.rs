use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelClient;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn fetch_clients(&self) -> Result<Vec<Client>, RepositoryError> {
        let query = "SELECT * FROM client;";
        let mut response= self.db.query(query).await?;
        let result: Vec<ModelClient> = response.take(0)?;
        let fdp = result.into_iter().map(|x| x.into()).collect();
        Ok(fdp)
    }
}
