use super::SharedState;

use crate::handler::{
    car::{handler_fetch_cars, handler_get_car},
    client::{
        handle_clients_table, handle_decrement_clients_paging, handle_increment_clients_paging,
        handle_search_client, handler_fetch_clients, handler_get_client,
    },
    home::handler_home,
};
use tower_http::services::fs::ServeDir;

use axum::{
    Router,
    routing::{get, post},
};

pub fn routes(app_state: SharedState) -> Router {
    let api_router = Router::new()
        .route("/", get(handler_home))
        .route("/clients", get(handler_fetch_clients))
        .route("/clients/table", get(handle_clients_table))
        .route(
            "/clients/decrement/paging",
            get(handle_decrement_clients_paging),
        )
        .route(
            "/clients/increment/paging",
            get(handle_increment_clients_paging),
        )
        .route("/clients/search", post(handle_search_client))
        .route("/client/{id}", get(handler_get_client))
        .route("/cars", get(handler_fetch_cars))
        .route("/cars/{id}", get(handler_get_car))
        .with_state(app_state);

    let static_router = Router::new().nest_service("/static", ServeDir::new("templates"));

    api_router.merge(static_router)
}
