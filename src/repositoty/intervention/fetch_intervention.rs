use std::fmt::Display;

use crate::{
    entity::Intervention,
    repositoty::{PagingFilter, Repository, RepositoryError, model::ModelIntervertion},
};

impl Repository {
    pub async fn fetch_intervention(
        &self,
        _paging: PagingFilter,
        _search_text: Option<String>,
        fillter: InterventionFilter,
    ) -> Result<Vec<Intervention>, RepositoryError> {
        let query = format!("select *,car_id.* from intervention {} ;", fillter);
        let mut response = self.db.query(query).await?;
        let result: Vec<ModelIntervertion> = response.take(0)?;
        result
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<Intervention>, _>>()
            .map_err(RepositoryError::ParsingError)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterventionFilter {
    client_id: Option<String>,
}

impl Display for InterventionFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.client_id {
            None => write!(f, ""),
            Some(client_id) => write!(f, "where client = client:{} ", client_id),
        }
    }
}
