use crate::repositoty::error::RepositoryError;

use super::super::super::entity::*;
use super::super::Repository;

impl Repository {
    pub fn get_client(&self) -> Result<Client, RepositoryError> {
        todo!()
    }
}
