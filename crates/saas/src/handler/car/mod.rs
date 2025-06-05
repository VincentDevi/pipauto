use super::super::SharedState;

use super::super::Error;
use crate::repositoty::{CarsFilter, Repository};
use axum::{
    Json,
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use common::car::*;
use templating::{Render, cars::*};

pub async fn handler_get_car(
    State(state): State<SharedState>,
    id: Path<String>,
) -> Result<Json<Car>, Error> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    let car_details = repository.get_car(id.to_string(), None).await?;
    Ok(Json(car_details))
}

pub async fn handler_fetch_cars(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, Error> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    let fetched_cars = repository
        .fetch_cars(None, None, CarsFilter::default())
        .await?;
    let prout = fetched_cars.into_iter().map(|x| x.into()).collect();
    let template = CarsTemplate { cars: prout };
    Ok(Html(template.render_template()?))
}
