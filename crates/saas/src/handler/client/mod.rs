use super::super::SharedState;
use crate::{
    common::{
        Address, ClientRecordId, Records,
        intervention::{InterventionType, Maintenance},
    },
    entity::{Car, Client, Intervention, SpecificInterventionWithCar},
    repositoty::{CarsFilter, InterventionFilter, PagingFilter, Repository},
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Deserializer, de};
use std::{fmt, str::FromStr};
use surrealdb::RecordId;

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

    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "client.html")]
pub struct ClientTemplatePage {
    id: String,
    full_name: String,
}

#[derive(Template)]
#[template(path = "client_details_page.html")]
pub struct ClientDetailsTemplatePage {
    id: String,
    full_name: String,
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
            name: value.full_name(),
            address: value.address().to_string(),
            phone: value.phone().map(|x| x.to_string()),
            email: value.email().map(|x| x.to_string()),
        }
    }
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
    Ok(Html(template.render()?))
}

#[derive(Debug, Deserialize)]
pub struct Body {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    search: Option<String>,
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

    Ok(Html(template.render()?))
}

#[derive(Debug, Deserialize)]
pub struct CreateClient {
    pub first_name: String,
    pub last_name: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    pub address: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub car: Option<String>,
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
    Ok(Html(template.render()?))
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
    Ok(Html(template.render()?))
}

#[derive(Debug, Deserialize)]
pub struct UpdateClient {
    pub first_name: String,
    pub last_name: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    pub address: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub car: Option<String>,
}

#[derive(Template)]
#[template(path = "client_create.html")]
pub struct CreateClientTemplate;

#[derive(Template)]
#[template(path = "client_update.html")]
pub struct UpdateClientTemplate {
    id: String,
    first_name: String,
    last_name: String,
    address: String,
    phone: Option<String>,
    email: Option<String>,
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
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "client_tab_details.html")]
pub struct ClientTabDetailTemplate {
    first_name: String,
    last_name: String,
    country: String,
    street: String,
    number: String,
    postal: String,
    phone: Option<String>,
    email: Option<String>,
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
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "client_tab_cars.html")]
pub struct ClientCar {
    brand: String,
    model: String,
    cc: String,
    fuel: String,
    year: String,
    oil_quantity: String,
    oil_type: String,
    interventions: Vec<ClientTabCarIntervention>,
}

impl From<Car> for ClientCar {
    fn from(value: Car) -> Self {
        Self {
            brand: value.brand().to_string(),
            model: value.model().to_string(),
            cc: value.cc().to_string(),
            fuel: value.fuel().to_string(),
            year: value.year().to_string(),
            oil_quantity: value.oil_quantity().to_string(),
            oil_type: value.oil_type().to_string(),
            interventions: value
                .intervention()
                .into_iter()
                .map(|x| x.clone().into())
                .collect(),
        }
    }
}

pub struct ClientTabCarIntervention {
    intervention_type: String,
    amount: String,
    milage: String,
    intervention_date: String,
}

impl From<Intervention> for ClientTabCarIntervention {
    fn from(value: Intervention) -> Self {
        Self {
            intervention_type: value.intervention_type().to_string(),
            amount: value.price().to_string(),
            milage: value.mileage().to_string(),
            intervention_date: value.intervention_date().to_string(),
        }
    }
}

pub struct InterventionTypeTemplate {
    repair: Option<String>,
    maintenance: Option<MaintenanceTemplate>,
}

impl From<InterventionType> for InterventionTypeTemplate {
    fn from(value: InterventionType) -> Self {
        match value {
            InterventionType::Repair => Self {
                repair: Some("Repair".to_string()),
                maintenance: None,
            },
            InterventionType::Maintenance(maintenance) => Self {
                repair: None,
                maintenance: Some(maintenance.into()),
            },
        }
    }
}

pub struct MaintenanceTemplate {
    filter_air: bool,
    filter_cabin: bool,
    filter_oil: bool,
    type_specific_maintenance: Option<String>,
}

impl From<Maintenance> for MaintenanceTemplate {
    fn from(value: Maintenance) -> Self {
        Self {
            filter_air: value.filter_air(),
            filter_cabin: value.filter_cabin(),
            filter_oil: value.filter_air(),
            type_specific_maintenance: value.type_specific_maintenance().map(|x| x.to_string()),
        }
    }
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
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "client_tab_history.html")]
pub struct ClientTabHistoryTemplate {
    interventions: Vec<InterventionHistoryTemplate>,
}

impl From<SpecificInterventionWithCar> for InterventionHistoryTemplate {
    fn from(value: SpecificInterventionWithCar) -> Self {
        Self {
            intervention_type: value.intervention_type().to_string(),
            intervention_date: value.intervention_date().to_string(),
            car: InterventionCarHistory {
                brand: value.car().brand.clone(),
                model: value.car().model.clone(),
            },
        }
    }
}

pub struct InterventionHistoryTemplate {
    intervention_type: String,
    intervention_date: String,
    car: InterventionCarHistory,
}

pub struct InterventionCarHistory {
    brand: String,
    model: String,
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
