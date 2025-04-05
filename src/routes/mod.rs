use crate::handler::{
    client::{handler_fetch_clients, handler_get_client},
    home::handler_home,
};
use tower_http::services::fs::ServeDir;

use super::AppState;
use axum::{Router, routing::get};

pub fn routes(app_state: AppState) -> Router {
    let api_router = Router::new()
        .route("/", get(handler_home))
        .route("/clients", get(handler_fetch_clients))
        .route("/clients/{:id}", get(handler_get_client))
        .with_state(app_state);

    let static_router = Router::new().nest_service("/static", ServeDir::new("templates"));

    api_router.merge(static_router)
}
