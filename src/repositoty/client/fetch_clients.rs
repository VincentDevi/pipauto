use crate::repositoty::PagingFilter;
use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelClient;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn fetch_clients(
        &self,
        paging: PagingFilter,
    ) -> Result<(Vec<Client>, u32), RepositoryError> {
        let query = format!(
            "return SELECT * FROM client {};
            return count(SELECT * FROM client);
            ",
            paging
        );

        let mut response = self.db.query(query).await?;
        let result: Vec<ModelClient> = response.take(0)?;
        let count: Option<u32> = response.take(1)?;

        let fetched_clients = result.into_iter().map(|x| x.into()).collect();

        Ok((fetched_clients, count.unwrap_or_default()))
    }
}
