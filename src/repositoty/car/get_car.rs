use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelCar;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn get_car(
        &self,
        id: String,
        client_id: Option<String>,
    ) -> Result<Car, RepositoryError> {
        let where_clause = match &client_id {
            Some(client_id) => &format!(" and client = client:{} ", client_id),
            None => "",
        };

        let query = format!(
            "select *,(select * from intervention where car_id = car:{} {}) as interventions from car:{}",
            id, where_clause, id
        );
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelCar> = response.take(0)?;
        let car = result
            .get(0)
            .cloned()
            .ok_or(RepositoryError::DatabaseError)?;
        car.try_into().map_err(RepositoryError::ParsingError)
    }
}
