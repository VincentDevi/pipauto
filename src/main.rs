mod common;
mod entity;
mod error;
mod repositoty;
mod routes;

use std::sync::Arc;

use error::Error;
use routes::*;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Surreal<Client>>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("warm").use_db("saas").await.unwrap();

    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener, routes(app_state)).await.unwrap();
    Ok(())
}
