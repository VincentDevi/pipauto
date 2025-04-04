use super::AppState;
use axum::Router;
use axum::routing::get;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "hello Hugo ! " }))
        .with_state(app_state)
}
