use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelCar;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn get_car(&self, id: String) -> Result<Car, RepositoryError> {
        let query = format!("SELECT * FROM car:{};", id);
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelCar> = response.take(0)?;
        let client = result.get(0).cloned().ok_or(RepositoryError::DatabaseError("Error in clone".into()))?;
        Ok(client.into())
    }
}