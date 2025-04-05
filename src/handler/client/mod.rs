use crate::{entity::Client, repositoty::Repository, AppState};
use axum::{extract::State, Json};

use super::error::*;
pub async fn handler_get_client() -> Result<Json<Client>, HandlerError> {
    todo!()
}

pub async fn handler_fetch_clients(State(state): State<AppState>) -> Result<Json<Vec<Client>>, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fdp = repository.fetch_clients().await?;
    Ok(Json(fdp))
}