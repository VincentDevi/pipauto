use crate::{AppState, entity::Client, repositoty::Repository};
use axum::{Json, extract::{State, Path}};

use super::error::*;
pub async fn handler_get_client(
    State(state): State<AppState>, id: Path<String>,
) -> Result<Json<Client>, HandlerError> {
    let db = state.db.lock().await;
    let repositoty = Repository::new(&db);
    let client_detail = repositoty.get_client(id.to_string()).await?;
    Ok(Json(client_detail))
}

pub async fn handler_fetch_clients(
    State(state): State<AppState>,
) -> Result<Json<Vec<Client>>, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients().await?;
    Ok(Json(fetched_clients))
}

