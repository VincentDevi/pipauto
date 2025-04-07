use crate::{
    AppState,
    entity::Client,
    repositoty::{PagingFilter, Repository},
};
use askama::Template;
use axum::{
    Json,
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
};

use super::error::*;
pub async fn handler_get_client(
    State(state): State<AppState>,
    id: Path<String>,
) -> Result<Json<Client>, HandlerError> {
    let db = state.db.lock().await;
    let repositoty = Repository::new(&db);
    let client_detail = repositoty.get_client(id.to_string()).await?;
    Ok(Json(client_detail))
}

pub async fn handler_fetch_clients(
    State(state): State<AppState>,
    pagination: Query<PagingFilter>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients(pagination.0).await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTemplate { clients: okok };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "clients.html")]
struct ClientsTemplate {
    clients: Vec<ClientTemp>,
}

#[derive(Debug, Clone)]
struct ClientTemp {
    name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
}

impl From<Client> for ClientTemp {
    fn from(value: Client) -> Self {
        Self {
            name: value.name(),
            address: value.address().to_string(),
            phone: value.phone().map(|x| x.to_string()),
            email: value.email().map(|x| x.to_string()),
        }
    }
}
