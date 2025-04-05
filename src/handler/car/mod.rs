use crate::{AppState, entity::Car, repositoty::Repository};
use axum::{extract::{Path, State}, Json};

use super::error::*;

pub async fn handler_get_car(
    State(state): State<AppState>, id: Path<String>,
) -> Result<Json<Car>, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let car_details = repository.get_car(id.to_string()).await?;
    Ok(Json(car_details))
}

pub async fn handler_fetch_cars(
    State(state): State<AppState>,
) -> Result<Json<Vec<Car>>, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_cars = repository.fetch_cars().await?;
    Ok(Json(fetched_cars))
}

