use crate::handler::client::{handler_fetch_clients, handler_get_client};
use crate::handler::car::{handler_fetch_cars, handler_get_car};

use super::AppState;
use axum::Router;
use axum::routing::get;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "hello Hugo ! " }))
        .route("/cars", get(handler_fetch_cars))
        .route("/cars/{id}", get(handler_get_car))
        .route("/clients", get(handler_fetch_clients))
        .route("/clients/{id}", get(handler_get_client))
        .with_state(app_state)
}
