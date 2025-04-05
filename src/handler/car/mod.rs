use crate::{AppState, entity::Car, repositoty::Repository};
use axum::{Json, extract::State};

use super::error::*;

pub async fn handler_fetch_cars(
    State(state): State<AppState>,
) -> Result<Json<Vec<Car>>, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_cars = repository.fetch_cars().await?;
    Ok(Json(fetched_cars))
}

