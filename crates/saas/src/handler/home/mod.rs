use askama::Template;
use axum::response::{Html, IntoResponse};

use super::HandlerError;

pub async fn handler_home() -> Result<impl IntoResponse, HandlerError> {
    let template = HomePage {
        name: "Filippo".to_string(),
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomePage {
    name: String,
}
