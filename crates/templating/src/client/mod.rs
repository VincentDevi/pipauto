mod client_create_template;
mod client_update_template;
mod details;

pub use client_create_template::*;
pub use client_update_template::*;
pub use details::*;

use askama::Template;

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
