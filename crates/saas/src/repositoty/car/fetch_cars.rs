use std::fmt::Display;

use super::super::Repository;
use crate::repositoty::PagingFilter;
use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelCar;
use common::car::*;

impl Repository {
    pub async fn fetch_cars(
        &self,
        _paging: Option<PagingFilter>,
        _search_text: Option<String>,
        filter: CarsFilter,
    ) -> Result<Vec<Car>, RepositoryError> {
        let query = format!(
            "SELECT *,(select * from intervention where car_id = $parent.id) as interventions FROM car {} ;",
            filter
        );
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelCar> = response.take(0)?;
        result
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<Car>, _>>()
            .map_err(RepositoryError::ParsingError)
    }
}

// TODO this will need to be refactored
// the whole filtering should be rethink to handle (eq, neq, in, out, etc).
// At the moment I am keeping it very simple just to make it work. This is not optimized at all.
// I should create my own orm in a way.
#[derive(Debug, Clone, Default)]
pub struct CarsFilter {
    client: Option<String>,
}

impl CarsFilter {
    pub fn new(client_id: Option<String>) -> Self {
        Self { client: client_id }
    }
}

impl Display for CarsFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.client {
            None => write!(f, ""),
            Some(client) => write!(f, "where client_id = client:{} ", client),
        }
    }
}
