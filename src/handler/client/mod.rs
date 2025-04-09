use std::sync::Arc;

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
use tokio::sync::RwLock;

use super::error::*;
pub async fn handler_get_client(
    State(state): State<Arc<RwLock<AppState>>>,
    id: Path<String>,
) -> Result<Json<Client>, HandlerError> {
    let state = state.read().await;
    let db = state.db.lock().await;
    let repositoty = Repository::new(&db);
    let client_detail = repositoty.get_client(id.to_string()).await?;
    Ok(Json(client_detail))
}

pub async fn handler_fetch_clients(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, HandlerError> {
    let state = state.read().await;
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository
        .fetch_clients(PagingFilter::new(state.offset, state.limit))
        .await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTemplate {
        clients: okok,
        paging: Paging {
            start: state.offset + 1,
            count: 100,
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "clients.html")]
struct ClientsTemplate {
    clients: Vec<ClientTemp>,
    paging: Paging,
}

#[derive(Debug, Clone, Copy)]
struct Paging {
    start: u32,
    count: u32,
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
    State(state): State<Arc<RwLock<AppState>>>,
    pagination: Query<PagingFilter>,
) -> Result<impl IntoResponse, HandlerError> {
    let state = state.read().await;
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients(pagination.0).await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: state.offset + 1,
            count: 100,
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "clients_table.html")]
struct ClientsTableTemplate {
    clients: Vec<ClientTemp>,
    paging: Paging,
}

pub async fn handle_increment_clients_paging(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, HandlerError> {
    let state = state.read().await;
    let db = state.db.lock().await;
    let repository = Repository::new(&db);
    let new_paging = PagingFilter::new(state.offset, state.limit);
    let fetched_clients = repository.fetch_clients(new_paging).await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: state.offset + 1,
            count: 100,
        },
    };
    println!("aaozheiahzeiuahr: {} ", state.offset);
    Ok(Html(template.render()?))
}

pub async fn handle_decrement_clients_paging(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, HandlerError> {
    println!("decrement handler");
    let state_read = state.read().await;
    let db = state_read.db.lock().await.clone();
    let old_paging = PagingFilter::new(state_read.offset, state_read.limit);
    drop(state_read);
    let repository = Repository::new(&db);
    let mut teeeest = state.write().await;
    teeeest.offset += old_paging.offset();
    drop(teeeest);
    let new_paging = old_paging.decrement_paging(1);

    let fetched_clients = repository.fetch_clients(new_paging).await?;
    let okok: Vec<ClientTemp> = fetched_clients.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: new_paging.offset() + 1,
            count: 100,
        },
    };
    println!("old paging : {:?}", old_paging);
    println!("new paging : {:?}", new_paging);
    println!("end of dercrement handler");
    Ok(Html(template.render()?))
}
