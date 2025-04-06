use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelCar;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn fetch_cars(&self) -> Result<Vec<Car>, RepositoryError> {
        let query = "SELECT * FROM car;";
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelCar> = response.take(0)?;
        let fetched_cars: Vec<Car> = result.into_iter().map(|x| x.into()).collect();
        Ok(fetched_cars)
    }
}
