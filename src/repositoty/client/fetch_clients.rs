use crate::repositoty::error::RepositoryError;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub fn fetch_clients(&self) -> Result<Vec<Client>, RepositoryError> {
        todo!()
    }
}
