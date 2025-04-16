use super::super::SharedState;

use crate::{entity::Client, repositoty::Repository};
use askama::Template;
use axum::{
    Json,
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Deserializer, de};
use std::{fmt, str::FromStr};

use super::error::*;
pub async fn handler_get_client(
    State(state): State<SharedState>,
    id: Path<String>,
) -> Result<Json<Client>, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repositoty = Repository::new(&db);
    let client_detail = repositoty.get_client(id.to_string()).await?;
    Ok(Json(client_detail))
}

pub async fn handler_fetch_clients(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();

    let paging = state.read().await.paging;

    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients(paging, None).await?;
    let okok: Vec<ClientTemp> = fetched_clients.0.into_iter().map(|x| x.into()).collect();
    let template = ClientsTemplate {
        clients: okok,
        paging: Paging {
            start: paging.offset() + 1,
            count: fetched_clients.1,
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
    State(state): State<SharedState>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();

    let where_clause = params.free_text;
    let paging = state.read().await.paging;

    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients(paging, where_clause).await?;

    let okok: Vec<ClientTemp> = fetched_clients.0.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: paging.offset() + 1,
            count: fetched_clients.1,
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
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();

    println!("begin increment : {}", state.read().await.paging.offset());
    let repository = Repository::new(&db);
    {
        let mut app_state = state.write().await;
        let current_paging = app_state.paging;
        app_state.paging = current_paging.increment_paging(current_paging.limit());
    }

    let paging = state.read().await.paging;
    let where_clause = None;

    let fetched_clients = repository.fetch_clients(paging, where_clause).await?;

    let okok: Vec<ClientTemp> = fetched_clients.0.into_iter().map(|x| x.into()).collect();

    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: paging.offset() + 1,
            count: fetched_clients.1,
        },
    };
    println!("end of increment : {} ", paging.offset());
    Ok(Html(template.render()?))
}

pub async fn handle_decrement_clients_paging(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();

    let repository = Repository::new(&db);

    {
        let mut app_state = state.write().await;
        let current_paging = app_state.paging;
        app_state.paging = current_paging.decrement_paging(current_paging.limit());
    }

    let paging = state.read().await.paging;
    let where_clause = None;

    let fetched_clients = repository.fetch_clients(paging, where_clause).await?;
    let okok: Vec<ClientTemp> = fetched_clients.0.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: paging.offset() + 1,
            count: fetched_clients.1,
        },
    };
    println!("end of decrement : {} ", paging.offset());
    Ok(Html(template.render()?))
}

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    free_text: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
