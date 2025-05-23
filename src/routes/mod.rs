use super::SharedState;

use crate::handler::{
    car::{handler_fetch_cars, handler_get_car},
    client::{
        handle_clients_table, handle_decrement_clients_paging, handle_increment_clients_paging,
        handle_search_client, handler_client_create, handler_client_tab_cars,
        handler_client_tab_details, handler_client_tab_history, handler_create_client_page,
        handler_fetch_clients, handler_get_client, handler_update_client,
        handler_update_client_page,
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
        .route("/client/create", get(handler_create_client_page))
        .route("/client/create", post(handler_client_create))
        .route("/client/update/{id}", get(handler_update_client_page))
        .route("/client/{id}/details", get(handler_client_tab_details))
        .route("/client/{id}/cars", get(handler_client_tab_cars))
        .route("/client/{id}/history", get(handler_client_tab_history))
        .route("/client/update/{id}", post(handler_update_client))
        .route("/cars", get(handler_fetch_cars))
        .route("/cars/{id}", get(handler_get_car))
        .with_state(app_state);

    let static_router = Router::new().nest_service("/static", ServeDir::new("templates"));

    api_router.merge(static_router)
}
