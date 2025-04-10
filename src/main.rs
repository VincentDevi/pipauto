mod common;
mod entity;
mod error;
mod handler;
mod repositoty;
mod routes;

use std::sync::Arc;

use error::Error;
use repositoty::PagingFilter;
use routes::*;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use tokio::sync::Mutex;
use tokio::{net::TcpListener, sync::RwLock};

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Debug)]
struct AppState {
    db: Arc<Mutex<Surreal<Client>>>,
    paging: PagingFilter,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("pipauto").use_db("saas").await.unwrap();

    let app_state = SharedState::new(RwLock::new(AppState {
        db: Arc::new(Mutex::new(db)),
        paging: PagingFilter::default(),
    }));

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener, routes(app_state)).await.unwrap();
    Ok(())
}
