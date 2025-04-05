use crate::entity::Client;
use axum::Json;

use super::error::*;
pub async fn handler_get_client() -> Result<Json<Client>, HandlerError> {
    todo!()
}
