use super::super::SharedState;
use crate::{
    entity::Client,
    repositoty::{CarsFilter, InterventionFilter, PagingFilter, Repository},
};
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use repository::record::{ClientRecordId, Records};
use surrealdb::RecordId;
use templating::{Body, Render, client::*, clients::*};

use super::error::*;
pub async fn handler_get_client(
    State(state): State<SharedState>,
    id: Path<String>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repositoty = Repository::new(&db);
    let client_detail = repositoty.get_client(&id).await?;
    let template = ClientTemplatePage {
        id: client_detail.id().id(),
        full_name: client_detail.full_name(),
    };

    Ok(Html(template.render_template()?))
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
    Ok(Html(template.render_template()?))
}

pub async fn handle_clients_table(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();

    let paging = state.read().await.paging;

    let repository = Repository::new(&db);
    let fetched_clients = repository.fetch_clients(paging, None).await?;

    let okok: Vec<ClientTemp> = fetched_clients.0.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: paging.offset() + 1,
            count: fetched_clients.1,
        },
    };
    Ok(Html(template.render_template()?))
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
    Ok(Html(template.render_template()?))
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
    Ok(Html(template.render_template()?))
}

pub async fn handle_search_client(
    State(state): State<SharedState>,
    Form(body): Form<Body>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    {
        let mut app_state = state.write().await;
        let current_paging = app_state.paging;
        app_state.paging = current_paging.reset();
    }
    let paging = state.read().await.paging;
    let where_clause = body.search;

    let fetched_clients = repository.fetch_clients(paging, where_clause).await?;
    let okok: Vec<ClientTemp> = fetched_clients.0.into_iter().map(|x| x.into()).collect();
    let template = ClientsTableTemplate {
        clients: okok,
        paging: Paging {
            start: paging.offset() + 1,
            count: fetched_clients.1,
        },
    };
    Ok(Html(template.render_template()?))
}

pub async fn handler_client_create(
    State(state): State<SharedState>,
    Form(form_data): Form<CreateClient>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    let new_client_record = repository.create_client(form_data.into()).await?;

    let template = ClientDetailsTemplatePage {
        id: new_client_record.id().id(),
        full_name: new_client_record.full_name(),
    };

    Ok(Html(template.render_template()?))
}

pub async fn handler_create_client_page(
    State(_state): State<SharedState>,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Html(CreateClientTemplate.render()?))
}

pub async fn handler_update_client_page(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();

    let repository = Repository::new(&db);
    let client = repository.get_client(&id).await?;

    let template = UpdateClientTemplate {
        id,
        first_name: client.first_name(),
        last_name: client.last_name(),
        address: client.address().to_string(),
        email: client.email().map(|x| x.to_string()),
        phone: client.phone().map(|x| x.to_string()),
    };
    Ok(Html(template.render_template()?))
}

pub async fn handler_update_client(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Form(form_data): Form<UpdateClient>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let client_record_id: RecordId = ClientRecordId::new(&id).into();
    let repository = Repository::new(&db);
    let updated_client: Client = repository
        .update_client(client_record_id, form_data)
        .await?;
    let template = ClientDetailsTemplatePage {
        id: updated_client.id().id(),
        full_name: updated_client.full_name(),
    };
    Ok(Html(template.render_template()?))
}

pub async fn handler_client_tab_details(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    let client = repository.get_client(&id).await?;
    let template = ClientTabDetailTemplate {
        first_name: client.first_name(),
        last_name: client.last_name(),
        country: client.address().country(),
        street: client.address().street(),
        postal: client.address().postal(),
        number: client.address().number(),
        phone: client.phone().map(|x| x.to_string()),
        email: client.email().map(|x| x.to_string()),
    };
    Ok(Html(template.render_template()?))
}

pub async fn handler_client_tab_cars(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    let car = repository
        .fetch_cars(None, None, CarsFilter::new(Some(id)))
        .await?;
    let template: ClientCar = car.first().unwrap().clone().into();
    Ok(Html(template.render_template()?))
}

pub async fn handler_client_tab_history(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, HandlerError> {
    let db = state.read().await.db.lock().await.clone();
    let repository = Repository::new(&db);
    let interventions = repository
        .fetch_intervention(
            PagingFilter::default(),
            None,
            InterventionFilter {
                client_id: Some(id),
            },
        )
        .await?;
    let template = ClientTabHistoryTemplate {
        interventions: interventions.into_iter().map(|x| x.into()).collect(),
    };
    Ok(Html(template.render_template()?))
}
