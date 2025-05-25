use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

use crate::{
    entity::SpecificInterventionWithCar,
    repositoty::{PagingFilter, Repository, RepositoryError, model::ModelInterventionType},
};

impl Repository {
    pub async fn fetch_intervention(
        &self,
        _paging: PagingFilter,
        _search_text: Option<String>,
        fillter: InterventionFilter,
    ) -> Result<Vec<SpecificInterventionWithCar>, RepositoryError> {
        let query = format!(
            "select *,car_id.id, car_id.brand, car_id.model from intervention {} ;",
            fillter
        );
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelIntervertionWithCar> = response.take(0)?;
        result
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<SpecificInterventionWithCar>, _>>()
            .map_err(RepositoryError::ParsingError)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterventionFilter {
    pub client_id: Option<String>,
}

impl Display for InterventionFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.client_id {
            None => write!(f, ""),
            Some(client_id) => write!(f, "where client = client:{} ", client_id),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelIntervertionWithCar {
    pub id: RecordId,
    pub client: RecordId,
    pub car_id: ModelInterventionCar,
    pub price: String,
    pub mileage: String,
    pub remark: Vec<String>,
    pub intervention_date: Datetime,
    pub intervention_type: ModelInterventionType,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
impl ModelIntervertionWithCar {
    pub fn intervention_date(&self) -> Datetime {
        self.intervention_date.clone()
    }
    pub fn intervention_type(&self) -> ModelInterventionType {
        self.intervention_type.clone()
    }
    pub fn created_at(&self) -> Datetime {
        self.created_at.clone()
    }
    pub fn id(&self) -> RecordId {
        self.id.clone()
    }
    pub fn client(&self) -> RecordId {
        self.client.clone()
    }
    pub fn car(&self) -> ModelInterventionCar {
        self.car_id.clone()
    }
    pub fn price(&self) -> String {
        self.price.clone()
    }
    pub fn mileage(&self) -> String {
        self.mileage.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelInterventionCar {
    pub id: RecordId,
    pub brand: String,
    pub model: String,
}
