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
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository
        .fetch_clients(PagingFilter::new(state.offset, state.limit))
        .await?;
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

pub async fn handle_clients_table(
    State(state): State<AppState>,
    pagination: Query<PagingFilter>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients(pagination.0).await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate { clients: okok };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "clients_table.html")]
struct ClientsTableTemplate {
    clients: Vec<ClientTemp>,
}

pub async fn handle_increment_clients_paging(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository
        .fetch_clients(PagingFilter::new(state.offset + state.limit, state.limit))
        .await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate { clients: okok };
    Ok(Html(template.render()?))
}

pub async fn handle_decrement_clients_paging(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.db.lock().await;
    let repository = Repository::new(&db);

    // Calculate new offset, ensuring it doesn't go below zero
    let new_offset = if state.offset >= state.limit {
        state.offset - state.limit
    } else {
        0
    };

    let fetched_clients = repository
        .fetch_clients(PagingFilter::new(new_offset, state.limit))
        .await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate { clients: okok };
    Ok(Html(template.render()?))
}
