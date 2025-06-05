use super::super::Error;
use axum::response::{Html, IntoResponse};
use templating::{Render, home::HomePage};

pub async fn handler_home() -> Result<impl IntoResponse, Error> {
    let template = HomePage {
        name: "Filippo".to_string(),
    };
    Ok(Html(template.render_template()?))
}
