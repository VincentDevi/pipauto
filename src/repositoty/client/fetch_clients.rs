use crate::repositoty::PagingFilter;
use crate::repositoty::error::RepositoryError;
use crate::repositoty::model::ModelClient;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub async fn fetch_clients(
        &self,
        paging: PagingFilter,
        search_text: Option<String>,
    ) -> Result<(Vec<Client>, u32), RepositoryError> {
        let where_clause = match &search_text {
            None => "".to_string(),
            Some(search_text) => format!(
                "where full_name @@ {} or email @@ {} or phone @@ {}",
                search_text, search_text, search_text
            ),
        };
        let (score, order_by) = match search_text {
            None => ("".to_string(), "".to_string()),
            Some(_) => (
                ",search::score(0) as score".to_string(),
                "order by score asc".to_string(),
            ),
        };
        let query = format!(
            "return SELECT * {} FROM client {} {} {};
            return count(SELECT * FROM client);
            ",
            score, paging, where_clause, order_by
        );

        let mut response = self.db.query(query).await?;
        let result: Vec<ModelClient> = response.take(0)?;
        let count: Option<u32> = response.take(1)?;

        let fetched_clients = result.into_iter().map(|x| x.into()).collect();

        Ok((fetched_clients, count.unwrap_or_default()))
    }
}
