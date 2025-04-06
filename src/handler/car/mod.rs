use crate::{AppState, entity::Car, repositoty::Repository};
use askama::Template;
use axum::{
    Json,
    extract::{Path, State},
    response::{Html, IntoResponse},
};

use super::error::*;

pub async fn handler_get_car(
    State(state): State<AppState>,
    id: Path<String>,
) -> Result<Json<Car>, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let car_details = repository.get_car(id.to_string()).await?;
    Ok(Json(car_details))
}

pub async fn handler_fetch_cars(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_cars = repository.fetch_cars().await?;
    let prout = fetched_cars.into_iter().map(|x| x.into()).collect();
    let template = CarsTemplate { cars: prout };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "cars.html")]
pub struct CarsTemplate {
    cars: Vec<CarsTemp>,
}

#[derive(Debug, Clone)]
pub struct CarsTemp {
    cc: String,
    brand: String,
    oil_type: String,
    oil_quantity: String,
    year: String,
}

impl From<Car> for CarsTemp {
    fn from(value: Car) -> Self {
        Self {
            cc: value.cc().to_string(),
            brand: value.brand().to_string(),
            oil_type: value.oil_qtype().to_string(),
            oil_quantity: value.oil_quantity().to_string(),
            year: value.year().to_string(),
        }
    }
}
